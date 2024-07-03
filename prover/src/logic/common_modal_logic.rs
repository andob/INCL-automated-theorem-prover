use box_macro::bx;
use crate::formula::{Formula, FormulaExtras};
use crate::formula::Formula::{Imply, Necessary, Non, Possible, StrictImply};
use crate::graph::Graph;
use crate::logic::{Logic, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct ModalLogicRules<LOGIC : Logic>
{
    possibility : Possibility<LOGIC>,
    necessity : Necessity<LOGIC>,
}

impl <LOGIC : Logic> ModalLogicRules<LOGIC>
{
    pub fn new(possibility : Possibility<LOGIC>, necessity : Necessity<LOGIC>) -> ModalLogicRules<LOGIC>
    {
        return ModalLogicRules { possibility, necessity };
    }
}

impl <LOGIC : Logic> LogicRule for ModalLogicRules<LOGIC>
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let logic_semantics = factory.get_logic().get_semantics();

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
                return self.possibility.apply(factory, node, p, extras);
            }

            Necessary(box p, extras) =>
            {
                return self.necessity.apply(factory, node, p, extras);
            }

            StrictImply(box p, box q, extras) =>
            {
                let p_imply_q = Imply(bx!(p.with(extras)), bx!(q.with(extras)), extras.clone());
                let necessary_p_imply_q = Necessary(bx!(p_imply_q), extras.clone());

                return self.necessity.apply(factory, node, &necessary_p_imply_q, extras);
            }

            Non(box StrictImply(box p, box q, _), extras) =>
            {
                let p_imply_q = Imply(bx!(p.with(extras)), bx!(q.with(extras)), extras.clone());
                let non_p_imply_q = logic_semantics.negate(&p_imply_q, extras);

                return self.possibility.apply(factory, node, &non_p_imply_q, extras);
            }

            _ => None
        }
    }
}

pub struct Possibility<LOGIC : Logic>
{
    pub is_applicable : fn(&RuleApplyFactory, &ProofTreeNode, &FormulaExtras) -> bool,
    pub add_missing_graph_vertices : fn(&LOGIC, &mut Graph) -> (),
}

impl <LOGIC : Logic> Possibility<LOGIC>
{
    pub fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, p : &Formula, extras : &FormulaExtras) -> Option<ProofSubtree>
    {
        if !(self.is_applicable)(factory, node, extras) { return None };

        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.as_any().downcast_ref::<LOGIC>().unwrap();

        let current_world = *factory.modality_graph.nodes.iter().max().unwrap();
        let forked_world = current_world.fork();

        factory.modality_graph.clear_log();
        factory.modality_graph.add_node(forked_world);
        factory.modality_graph.add_vertex(current_world, forked_world);

        (self.add_missing_graph_vertices)(logic, factory.modality_graph);

        let p_in_forked_world = p.with(extras).in_world(forked_world);
        let p_in_forked_world_node = factory.new_node(p_in_forked_world);

        let comment = Formula::Comment(factory.modality_graph.log.clone());
        let comment_node = factory.new_node(comment);

        return Some(ProofSubtree::with_middle_vertical_nodes(vec![comment_node, p_in_forked_world_node]));
    }
}

pub struct Necessity<LOGIC : Logic>
{
    pub is_applicable : fn(&RuleApplyFactory, &ProofTreeNode) -> bool,
    pub dummy : fn(&LOGIC) -> ()
}

impl <LOGIC : Logic> Necessity<LOGIC>
{
    pub fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, p : &Formula, extras : &FormulaExtras) -> Option<ProofSubtree>
    {
        if !(self.is_applicable)(factory, node) { return None };

        //todo use dummy field or remove <LOGIC> type parameter
        //todo implement reapplication on each new vertex
        //todo if P,wi was already spawned from []P,wi, do not spawn it again!

        let formulas = factory.modality_graph.vertices.iter()
            .filter(|vertex| vertex.from == extras.possible_world)
            .map(|vertex| p.with(extras).in_world(vertex.to))
            .collect::<Vec<Formula>>();

        let mut nodes : Vec<ProofTreeNode> = vec![];
        for formula in formulas
        {
            nodes.push(factory.new_node(formula));
        }

        return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
    }
}
