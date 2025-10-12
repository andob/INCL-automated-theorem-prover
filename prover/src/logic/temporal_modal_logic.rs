use std::any::Any;
use std::collections::BTreeSet;
use box_macro::bx;
use crate::default_log_line_formatter;
use crate::formula::Formula::{InFuture, InPast, Necessary, Non, Possible};
use crate::graph::{Graph, GraphVertex};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection, LogicRuleResult};
use crate::logic::common_modal_logic::{Modality, ModalityRef};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 3, 14-17
pub struct TemporalModalLogic
{
    pub is_extended : bool
}

impl TemporalModalLogic
{
    pub fn basic() -> TemporalModalLogic { TemporalModalLogic { is_extended:false } }
    pub fn extended() -> TemporalModalLogic { TemporalModalLogic { is_extended:true } }
}

impl Logic for TemporalModalLogic
{
    fn get_name(&self) -> LogicName
    {
        return if !self.is_extended { LogicName::of("KTemporalModalLogic") }
        else { LogicName::of("KTemporalExtModalLogic") };
    }

    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinaryLogicSemantics {});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::InPast, TokenTypeID::InFuture,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(TemporalModalLogicRules::new(self.get_modality())),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl TemporalModalLogic
{
    pub fn get_modality(&self) -> Modality<TemporalModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| true,
            is_necessity_applicable: |_, _, _| true,
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_extended
                {
                    graph.add_missing_temporal_convergence_vertices();
                }
            },
        };
    }
}

struct TemporalModalLogicRules
{
    modality : Modality<TemporalModalLogic>
}

impl TemporalModalLogicRules
{
    pub fn new(modality : Modality<TemporalModalLogic>) -> TemporalModalLogicRules
    {
        return TemporalModalLogicRules { modality };
    }
}

impl LogicRule for TemporalModalLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> LogicRuleResult
    {
        return match &node.formula
        {
            Non(box Possible(box InPast(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_past = InPast(bx!(non_p), extras.clone());
                let necessary_non_p = Necessary(bx!(non_p_in_past), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);

                return LogicRuleResult::Subtree(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Possible(box InFuture(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_future = InFuture(bx!(non_p), extras.clone());
                let necessary_non_p = Necessary(bx!(non_p_in_future), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);

                return LogicRuleResult::Subtree(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Necessary(box InPast(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_past = InPast(bx!(non_p), extras.clone());
                let possible_non_p = Possible(bx!(non_p_in_past), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);

                return LogicRuleResult::Subtree(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Non(box Necessary(box InFuture(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_future = InFuture(bx!(non_p), extras.clone());
                let possible_non_p = Possible(bx!(non_p_in_future), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);

                return LogicRuleResult::Subtree(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Possible(box InPast(box p, _), extras) =>
            {
                factory.modality_graph.invert_all_vertices();
                factory.modality_graph.set_log_line_formatter(bx!(|v| format!("{}R{}\n", v.to, v.from)));

                let subtree = self.modality.apply_possibility(factory, node, p, extras);

                factory.modality_graph.invert_all_vertices();
                factory.modality_graph.set_log_line_formatter(default_log_line_formatter!());

                return subtree;
            }

            Possible(box InFuture(box p, _), extras) =>
            {
                return self.modality.apply_possibility(factory, node, p, extras);
            }

            Necessary(box InPast(box p, _), extras) =>
            {
                factory.modality_graph.invert_all_vertices();
                factory.modality_graph.set_log_line_formatter(bx!(|v| format!("{}R{}\n", v.to, v.from)));

                let subtree = self.modality.apply_necessity(factory, node, p, extras);

                factory.modality_graph.invert_all_vertices();
                factory.modality_graph.set_log_line_formatter(default_log_line_formatter!());

                return subtree;
            }

            Necessary(box InFuture(box p, _), extras) =>
            {
                return self.modality.apply_necessity(factory, node, p, extras);
            }

            _ => LogicRuleResult::Empty
        };
    }
}

impl Graph
{
    fn invert_all_vertices(&mut self)
    {
        let inverted_vertices = self.vertices()
            .map(|vertex| GraphVertex::new(vertex.to, vertex.from))
            .collect::<BTreeSet<GraphVertex>>();

        self.set_vertices(inverted_vertices);
    }

    fn add_missing_temporal_convergence_vertices(&mut self)
    {
        let mut forward_vertices_to_add : Vec<GraphVertex> = vec![];
        let mut backward_vertices_to_add : Vec<GraphVertex> = vec![];

        for i_vertex in self.vertices()
        {
            for j_vertex in self.vertices()
            {
                if i_vertex != j_vertex && i_vertex.from == j_vertex.from
                {
                    let forward_convergent_vertex = GraphVertex::new(i_vertex.to, j_vertex.to);
                    let alternate_forward_convergent_vertex = GraphVertex::new(j_vertex.to, i_vertex.to);
                    if !forward_vertices_to_add.contains(&alternate_forward_convergent_vertex)
                    {
                        forward_vertices_to_add.push(forward_convergent_vertex);
                    }
                }

                if i_vertex != j_vertex && i_vertex.to == j_vertex.to
                {
                    let backward_convergent_vertex = GraphVertex::new(i_vertex.from, j_vertex.from);
                    let alternate_backward_convergent_vertex = GraphVertex::new(j_vertex.from, i_vertex.from);
                    if !backward_vertices_to_add.contains(&alternate_backward_convergent_vertex)
                    {
                        backward_vertices_to_add.push(backward_convergent_vertex);
                    }
                }
            }
        }

        self.set_log_line_formatter(bx!(|v| format!("{}φ{}\n", v.from, v.to)));
        self.add_vertices(forward_vertices_to_add);

        self.set_log_line_formatter(bx!(|v| format!("{}β{}\n", v.from, v.to)));
        self.add_vertices(backward_vertices_to_add);

        self.set_log_line_formatter(default_log_line_formatter!());
    }
}
