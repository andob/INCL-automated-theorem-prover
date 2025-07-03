use std::collections::BTreeSet;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::{Formula, FormulaExtras, PossibleWorld};
use crate::formula::Formula::{Imply, Necessary, Non, Possible, StrictImply};
use crate::graph::{Graph, GraphVertex};
use crate::logic::{Logic, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::subtree::ProofSubtree;

pub struct ModalLogicRules<LOGIC : Logic>
{
    modality : Rc<Modality<LOGIC>>
}

impl <LOGIC : Logic> ModalLogicRules<LOGIC>
{
    pub fn new(modality : Rc<Modality<LOGIC>>) -> ModalLogicRules<LOGIC>
    {
        return ModalLogicRules { modality };
    }
}

impl <LOGIC : Logic> LogicRule for ModalLogicRules<LOGIC>
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Possible(box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let necessary_non_p = Necessary(bx!(non_p), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);

                return Some(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Necessary(box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let possible_non_p = Possible(bx!(non_p), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);

                return Some(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Possible(box p, extras) =>
            {
                let p_with_parent_sign = p.with_sign(extras.sign);
                return self.modality.apply_possibility(factory, node, &p_with_parent_sign, extras);
            }

            Necessary(box p, extras) =>
            {
                let p_with_parent_sign = p.with_sign(extras.sign);
                return self.modality.apply_necessity(factory, node, &p_with_parent_sign, extras);
            }

            StrictImply(box p, box q, extras) =>
            {
                let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone());

                return self.modality.apply_necessity(factory, node, &p_imply_q, extras);
            }

            Non(box StrictImply(box p, box q, _), extras) =>
            {
                let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone());
                let non_p_imply_q = Non(bx!(p_imply_q.clone()), extras.clone());

                return self.modality.apply_possibility(factory, node, &non_p_imply_q, extras);
            }

            _ => None
        }
    }
}

#[derive(Clone)]
pub struct NecessityReapplicationData
{
    pub input_formula : Formula,
    pub input_possible_world : PossibleWorld,
    pub input_spawner_node_id : ProofTreeNodeID,
    pub input_leafs_node_ids : Vec<ProofTreeNodeID>,
    pub already_iterated_possible_worlds : Vec<PossibleWorld>,
}

pub struct Modality<LOGIC : Logic>
{
    pub is_possibility_applicable : fn(&RuleApplyFactory, &ProofTreeNode, &FormulaExtras) -> bool,
    pub is_necessity_applicable : fn(&RuleApplyFactory, &ProofTreeNode, &FormulaExtras) -> bool,
    pub add_missing_graph_vertices : fn(&LOGIC, &mut Graph) -> (),
}

impl <LOGIC : Logic> Modality<LOGIC>
{
    fn initialize_graph_if_needed(&self, factory : &mut RuleApplyFactory)
    {
        if factory.modality_graph.is_empty()
        {
            let logic_pointer = factory.get_logic().clone();
            let logic = logic_pointer.cast_to::<LOGIC>().unwrap();

            factory.modality_graph.add_node(PossibleWorld::zero());

            (self.add_missing_graph_vertices)(&logic, factory.modality_graph);

            factory.modality_graph.flush_log();
        }
    }

    pub fn apply_possibility(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        if !(self.is_possibility_applicable)(factory, node, extras) { return None };

        self.initialize_graph_if_needed(factory);

        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<LOGIC>()?;

        let current_world = extras.possible_world;
        let forked_world = factory.modality_graph.nodes().max()?.fork();

        factory.modality_graph.add_node(forked_world);
        factory.modality_graph.add_vertex(GraphVertex::new(current_world, forked_world));

        (self.add_missing_graph_vertices)(logic, factory.modality_graph);

        let p_in_forked_world = p.in_world(forked_world);
        let p_in_forked_world_node = factory.new_node(p_in_forked_world);

        let comment = Formula::Comment(factory.modality_graph.flush_log());
        let comment_node = factory.new_node(comment);

        let mut output_nodes = vec![comment_node, p_in_forked_world_node];
        self.reapply_necessity_after_possibility(factory, node, forked_world, &mut output_nodes);

        return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
    }

    pub fn apply_necessity(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        if !(self.is_necessity_applicable)(factory, node, extras) { return None };

        self.initialize_graph_if_needed(factory);

        let paths = factory.tree.get_paths_that_goes_through_node(node);
        let leaf_node_ids = paths.iter().map(|path| path.get_leaf_node_id()).collect();

        let mut reapplication_data = NecessityReapplicationData
        {
            input_formula: p.in_world(extras.possible_world),
            input_possible_world: extras.possible_world,
            input_spawner_node_id: node.id,
            input_leafs_node_ids: leaf_node_ids,
            already_iterated_possible_worlds: vec![],
        };

        let output_nodes = self.reapply_necessity(factory, &mut reapplication_data, PossibleWorld::zero());
        factory.push_necessity_reapplication(reapplication_data);

        return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
    }

    fn reapply_necessity_after_possibility(
        &self, factory : &mut RuleApplyFactory,
        node : &ProofTreeNode, forked_world : PossibleWorld,
        output_nodes : &mut Vec<ProofTreeNode>,
    )
    {
        let mut reusable_necessity_reapplications : Vec<NecessityReapplicationData> = vec![];

        while let Some(mut reapplication) = factory.pop_next_necessity_reapplication()
        {
            for path in factory.tree.get_paths_that_goes_through_node(node)
            {
                //necessary reapplication should happen only if we're on one of some specific paths
                if reapplication.input_leafs_node_ids.iter().any(|leaf_node_id| path.contains_node_with_id(*leaf_node_id))
                {
                    let mut output_nodes_from_necessity = self.reapply_necessity(factory, &mut reapplication, forked_world);
                    output_nodes.append(&mut output_nodes_from_necessity);
                }
            }

            reusable_necessity_reapplications.push(reapplication);
        }

        factory.push_necessity_reapplications(reusable_necessity_reapplications);
    }

    fn reapply_necessity(&self,
        factory : &mut RuleApplyFactory,
        reapplication_data : &mut NecessityReapplicationData,
        forked_world : PossibleWorld,
    ) -> Vec<ProofTreeNode>
    {
        let mut output_nodes : Vec<ProofTreeNode> = vec![];
        let output_formulas = factory.modality_graph.vertices()
            .filter(|vertex| vertex.from == reapplication_data.input_possible_world)
            .filter(|vertex| !reapplication_data.already_iterated_possible_worlds.contains(&vertex.to))
            .map(|vertex| reapplication_data.input_formula.in_world(vertex.to))
            .collect::<Vec<Formula>>();

        if !output_formulas.is_empty()
        {
            factory.set_spawner_node_id(Some(reapplication_data.input_spawner_node_id));

            let mut possible_worlds_on_tree_path = factory.tree.get_all_paths().iter()
                .filter(|path| reapplication_data.input_leafs_node_ids.iter()
                    .any(|leaf_id| path.contains_node_with_id(*leaf_id)))
                .flat_map(|path| path.nodes.iter()
                    .map(|node| node.formula.get_possible_world()))
                .collect::<BTreeSet<PossibleWorld>>();
            possible_worlds_on_tree_path.insert(forked_world);

            for formula in output_formulas
            {
                reapplication_data.already_iterated_possible_worlds.push(formula.get_possible_world());

                if possible_worlds_on_tree_path.contains(&formula.get_possible_world())
                {
                    output_nodes.push(factory.new_node(formula));
                }
            }
        }

        return output_nodes;
    }

    pub fn was_necessity_already_applied(&self, factory : &mut RuleApplyFactory, p : &Formula) -> bool
    {
        let p_with_stripped_extras = p.with_stripped_extras();
        return factory.modality_graph.necessity_reapplications()
            .map(|reapplication| reapplication.input_formula.with_stripped_extras())
            .any(|reapplication_formula| reapplication_formula == p_with_stripped_extras);
    }
}

pub struct ModalityRef
{
    apply_possibility_ref : Box<dyn Fn(&mut RuleApplyFactory, &ProofTreeNode, &Formula, &FormulaExtras) -> Option<ProofSubtree>>,
    apply_necessity_ref : Box<dyn Fn(&mut RuleApplyFactory, &ProofTreeNode, &Formula, &FormulaExtras) -> Option<ProofSubtree>>,
    was_necessity_already_applied_ref : Box<dyn Fn(&mut RuleApplyFactory, &Formula) -> bool>,
}

impl ModalityRef
{
    pub fn new<LOGIC : Logic>(modality : Modality<LOGIC>) -> ModalityRef
    {
        let modality_pointer1 = Rc::new(modality);
        let modality_pointer2 = modality_pointer1.clone();
        let modality_pointer3 = modality_pointer1.clone();
        return ModalityRef
        {
            apply_possibility_ref: Box::new(move |factory, node, p, extras|
                { modality_pointer1.apply_possibility(factory, node, p, extras) }),
            apply_necessity_ref: Box::new(move |factory, node, p, extras|
                { modality_pointer2.apply_necessity(factory, node, p, extras) }),
            was_necessity_already_applied_ref: Box::new(move |factory, p|
                { modality_pointer3.was_necessity_already_applied(factory, p) }),
        }
    }

    pub fn apply_possibility(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, p : &Formula, extras : &FormulaExtras) -> Option<ProofSubtree>
    {
        return (self.apply_possibility_ref)(factory, node, p, extras);
    }

    pub fn apply_necessity(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, p : &Formula, extras : &FormulaExtras) -> Option<ProofSubtree>
    {
        return (self.apply_necessity_ref)(factory, node, p, extras);
    }

    pub fn was_necessity_already_applied(&self, factory : &mut RuleApplyFactory, p : &Formula) -> bool
    {
        return (self.was_necessity_already_applied_ref)(factory, p);
    }
}
