use std::any::Any;
use box_macro::bx;
use crate::default_log_line_formatter;
use crate::formula::Formula::{InFuture, InPast, Necessary, Non, Possible};
use crate::formula::PossibleWorld;
use crate::graph::{Graph, GraphVertex};
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::Modality;
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 3
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
        return if !self.is_extended { LogicName::KTemporalModalLogic }
        else { LogicName::KTemporalExtModalLogic };
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
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::InPast, TokenTypeID::InFuture,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(TemporalModalLogicRules::new(self.get_modality())),
        ];
    }
}

impl TemporalModalLogic
{
    pub fn get_modality(&self) -> Modality<TemporalModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| { true },
            is_necessity_applicable: |_, _, _| { true },
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_extended
                {
                    graph.add_missing_forward_temporal_convergence_vertices();
                    graph.add_missing_backward_temporal_convergence_vertices();
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
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if factory.modality_graph.is_empty()
        {
            factory.modality_graph.add_node(PossibleWorld::zero());
        }

        return match &node.formula
        {
            Non(box Possible(box InPast(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_past = InPast(bx!(non_p), extras.clone());
                let necessary_non_p = Necessary(bx!(non_p_in_past), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);
                return Some(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Possible(box InFuture(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_future = InFuture(bx!(non_p), extras.clone());
                let necessary_non_p = Necessary(bx!(non_p_in_future), extras.clone());
                let necessary_non_p_node = factory.new_node(necessary_non_p);
                return Some(ProofSubtree::with_middle_node(necessary_non_p_node));
            }

            Non(box Necessary(box InPast(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_past = InPast(bx!(non_p), extras.clone());
                let possible_non_p = Possible(bx!(non_p_in_past), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);
                return Some(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Non(box Necessary(box InFuture(box p, _), _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_in_future = InFuture(bx!(non_p), extras.clone());
                let possible_non_p = Possible(bx!(non_p_in_future), extras.clone());
                let possible_non_p_node = factory.new_node(possible_non_p);
                return Some(ProofSubtree::with_middle_node(possible_non_p_node));
            }

            Possible(box InPast(box p, _), extras) =>
            {
                factory.modality_graph.set_log_line_formatter(|v| format!("{}R{}\n", v.to, v.from));
                factory.modality_graph.invert_all_vertices();

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
                factory.modality_graph.set_log_line_formatter(|v| format!("{}R{}\n", v.to, v.from));
                factory.modality_graph.invert_all_vertices();

                let subtree = self.modality.apply_necessity(factory, node, p, extras);

                factory.modality_graph.invert_all_vertices();
                factory.modality_graph.set_log_line_formatter(default_log_line_formatter!());

                return subtree;
            }

            Necessary(box InFuture(box p, _), extras) =>
            {
                return self.modality.apply_necessity(factory, node, p, extras);
            }

            _ => None
        };
    }
}

impl Graph
{
    fn invert_all_vertices(&mut self)
    {
        let inverted_vertices = self.vertices.iter()
            .map(|vertex| GraphVertex::new(vertex.to, vertex.from))
            .collect::<Vec<GraphVertex>>();

        self.vertices.clear();
        for inverted_vertex in inverted_vertices
        {
            self.vertices.insert(inverted_vertex);
        }
    }

    fn add_missing_forward_temporal_convergence_vertices(&mut self)
    {
        let mut vertices_to_add : Vec<GraphVertex> = vec![];

        for i_vertex in &self.vertices
        {
            for j_vertex in &self.vertices
            {
                if i_vertex != j_vertex && i_vertex.from == j_vertex.from
                {
                    let convergent_vertex = GraphVertex::new(i_vertex.to, j_vertex.to);
                    let alternate_convergent_vertex = GraphVertex::new(j_vertex.to, i_vertex.to);
                    if !vertices_to_add.contains(&alternate_convergent_vertex)
                    {
                        vertices_to_add.push(convergent_vertex);
                    }
                }
            }
        }

        self.set_log_line_formatter(|v| format!("{}φ{}\n", v.from, v.to));
        self.add_vertices(vertices_to_add);

        self.set_log_line_formatter(default_log_line_formatter!());
    }

    fn add_missing_backward_temporal_convergence_vertices(&mut self)
    {
        let mut vertices_to_add : Vec<GraphVertex> = vec![];

        for i_vertex in &self.vertices
        {
            for j_vertex in &self.vertices
            {
                if i_vertex != j_vertex && i_vertex.to == j_vertex.to
                {
                    let convergent_vertex = GraphVertex::new(i_vertex.from, j_vertex.from);
                    let alternate_convergent_vertex = GraphVertex::new(j_vertex.from, i_vertex.from);
                    if !vertices_to_add.contains(&alternate_convergent_vertex)
                    {
                        vertices_to_add.push(convergent_vertex);
                    }
                }
            }
        }

        self.set_log_line_formatter(|v| format!("{}β{}\n", v.from, v.to));
        self.add_vertices(vertices_to_add);

        self.set_log_line_formatter(default_log_line_formatter!());
    }
}
