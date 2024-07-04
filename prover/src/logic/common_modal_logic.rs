use box_macro::bx;
use crate::formula::{Formula, FormulaExtras, PossibleWorld};
use crate::formula::Formula::{Imply, Necessary, Non, Possible, StrictImply};
use crate::graph::Graph;
use crate::logic::{Logic, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::subtree::ProofSubtree;

pub struct ModalLogicRules<LOGIC : Logic>
{
    modality : Modality<LOGIC>
}

impl <LOGIC : Logic> ModalLogicRules<LOGIC>
{
    pub fn new(modality : Modality<LOGIC>) -> ModalLogicRules<LOGIC>
    {
        return ModalLogicRules { modality };
    }
}

impl <LOGIC : Logic> LogicRule for ModalLogicRules<LOGIC>
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let logic_semantics = factory.get_logic().get_semantics();

        self.initialize_modality_graph(factory);

        return match &node.formula
        {
            Non(box Possible(box p, _), extras) =>
            {
                let necessary_non_p = Necessary(bx!(logic_semantics.negate(p, extras)), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);
                return Some(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Necessary(box p, _), extras) =>
            {
                let possible_non_p = Possible(bx!(logic_semantics.negate(p, extras)), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);
                return Some(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Possible(p, extras) =>
            {
                return self.modality.apply_possibility(factory, node, p, extras);
            }

            Necessary(box p, extras) =>
            {
                return self.modality.apply_necessity(factory, node, p, extras);
            }

            StrictImply(box p, box q, extras) =>
            {
                let p_imply_q = Imply(bx!(p.with(extras)), bx!(q.with(extras)), extras.clone());

                return self.modality.apply_necessity(factory, node, &p_imply_q, extras);
            }

            Non(box StrictImply(box p, box q, _), extras) =>
            {
                let p_imply_q = Imply(bx!(p.with(extras)), bx!(q.with(extras)), extras.clone());
                let non_p_imply_q = logic_semantics.negate(&p_imply_q, extras);

                return self.modality.apply_possibility(factory, node, &non_p_imply_q, extras);
            }

            _ => None
        }
    }
}

impl <LOGIC: Logic> ModalLogicRules<LOGIC>
{
    fn initialize_modality_graph(&self, factory : &mut RuleApplyFactory)
    {
        if factory.modality_graph.is_empty()
        {
            let logic_pointer = factory.get_logic().clone();
            let logic = logic_pointer.as_any().downcast_ref::<LOGIC>().unwrap();

            factory.modality_graph.add_node(PossibleWorld::zero());

            (self.modality.add_missing_graph_vertices)(&logic, factory.modality_graph);
        }
    }
}

#[derive(Clone)]
pub struct NecessityReapplicationData
{
    pub input_formula : Formula,
    pub input_possible_world : PossibleWorld,
    pub input_spawner_node_id : ProofTreeNodeID,
    pub input_path_leaf_node_id : ProofTreeNodeID,
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
    pub fn apply_possibility(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        if !(self.is_possibility_applicable)(factory, node, extras) { return None };

        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.as_any().downcast_ref::<LOGIC>().unwrap();

        let current_world = extras.possible_world;
        let forked_world = factory.modality_graph.nodes.iter().max().unwrap().fork();

        factory.modality_graph.add_node(forked_world);
        factory.modality_graph.add_vertex(current_world, forked_world);

        (self.add_missing_graph_vertices)(logic, factory.modality_graph);

        let p_in_forked_world = p.with(extras).in_world(forked_world);
        let p_in_forked_world_node = factory.new_node(p_in_forked_world);

        let comment = Formula::Comment(factory.modality_graph.flush_log());
        let comment_node = factory.new_node(comment);

        let mut output_nodes = vec![comment_node, p_in_forked_world_node];
        self.reapply_necessity_after_possibility(factory, node, &mut output_nodes);

        return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
    }

    pub fn apply_necessity(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        if !(self.is_necessity_applicable)(factory, node, extras) { return None };

        let path = factory.tree.get_path_that_goes_through_node(node);
        let leaf_node_id = path.nodes.last().unwrap().id;

        let mut reapplication_data = NecessityReapplicationData
        {
            input_formula: p.with(extras),
            input_possible_world: extras.possible_world,
            input_spawner_node_id: node.id,
            input_path_leaf_node_id: leaf_node_id,
            already_iterated_possible_worlds: vec![],
        };

        let output_nodes = self.reapply_necessity(factory, &mut reapplication_data);
        factory.push_necessity_reapplication(reapplication_data);

        return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
    }

    fn reapply_necessity_after_possibility(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        output_nodes : &mut Vec<ProofTreeNode>)
    {
        let mut reusable_necessity_reapplications: Vec<NecessityReapplicationData> = vec![];

        while let Some(mut reapplication) = factory.pop_next_necessity_reapplication()
        {
            let path = factory.tree.get_path_that_goes_through_node(node);
            if path.contains_node_with_id(reapplication.input_path_leaf_node_id)
            {
                let mut output_nodes_from_necessity = self.reapply_necessity(factory, &mut reapplication);
                output_nodes.append(&mut output_nodes_from_necessity);
            }

            reusable_necessity_reapplications.push(reapplication);
        }

        factory.push_necessity_reapplications(reusable_necessity_reapplications);
    }

    fn reapply_necessity(&self,
        factory : &mut RuleApplyFactory,
        reapplication_data : &mut NecessityReapplicationData,
    ) -> Vec<ProofTreeNode>
    {
        let mut output_nodes : Vec<ProofTreeNode> = vec![];
        let output_formulas = factory.modality_graph.vertices.iter()
            .filter(|vertex| vertex.from == reapplication_data.input_possible_world)
            .filter(|vertex| !reapplication_data.already_iterated_possible_worlds.contains(&vertex.to))
            .map(|vertex| reapplication_data.input_formula.in_world(vertex.to))
            .collect::<Vec<Formula>>();

        if !output_formulas.is_empty()
        {
            factory.tree_node_factory.set_spawner_node_id(reapplication_data.input_spawner_node_id);

            for formula in output_formulas
            {
                reapplication_data.already_iterated_possible_worlds.push(formula.get_possible_world());
                output_nodes.push(factory.new_node(formula));
            }
        }

        return output_nodes;
    }
}
