use std::any::Any;
use std::collections::BTreeSet;
use box_macro::bx;
use crate::default_log_line_formatter;
use crate::formula::Formula::{And, Conditional, Necessary, Non, Possible};
use crate::formula::to_string::FormulaFormatOptions;
use crate::graph::GraphVertex;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::Modality;
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 5
pub struct ConditionalModalLogic
{
    pub is_extended : bool
}

impl ConditionalModalLogic
{
    pub fn basic() -> ConditionalModalLogic { ConditionalModalLogic { is_extended:false } }
    pub fn extended() -> ConditionalModalLogic { ConditionalModalLogic { is_extended:true } }
}

impl Logic for ConditionalModalLogic
{
    fn get_name(&self) -> LogicName
    {
        return if !self.is_extended { LogicName::ConditionalModalLogic }
        else { LogicName::ConditionalExtModalLogic };
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
            TokenTypeID::Necessary, TokenTypeID::Possible, TokenTypeID::Conditional,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(ConditionalModalLogicRules::new(self.get_modality())),
        ]
    }
}

impl ConditionalModalLogic
{
    pub fn get_modality(&self) -> Modality<ConditionalModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| true,
            is_necessity_applicable: |_, _, _| true,
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_extended
                {
                    graph.add_missing_reflexive_vertices();
                }
            },
        };
    }
}

struct ConditionalModalLogicRules
{
    modality : Modality<ConditionalModalLogic>
}

impl ConditionalModalLogicRules
{
    pub fn new(modality : Modality<ConditionalModalLogic>) -> ConditionalModalLogicRules
    {
        return ConditionalModalLogicRules { modality };
    }
}

impl LogicRule for ConditionalModalLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

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

            Possible(p, extras) =>
            {
                return self.modality.apply_possibility(factory, node, p, extras);
            }

            Necessary(box p, extras) =>
            {
                return self.modality.apply_necessity(factory, node, p, extras);
            }

            Non(box Conditional(box p, box q, _), extras) =>
            {
                let logic_pointer = factory.get_logic().clone();
                let logic = logic_pointer.as_any().downcast_ref::<ConditionalModalLogic>().unwrap();

                let mut formula_format_options = FormulaFormatOptions::default();
                formula_format_options.should_show_possible_worlds = false;
                let p_as_string = p.to_string_with_options(&formula_format_options);
                let p_as_string_cloned = p_as_string.clone();

                factory.modality_graph.set_log_line_formatter(bx!(move |v|
                    format!("{}R{} [{}]\n", v.from, v.to, p_as_string_cloned)));

                let subtree = if logic.is_extended
                {
                    let non_q = Non(bx!(q.clone()), extras.clone());
                    let non_q_and_p = And(bx!(non_q), bx!(p.clone()), extras.clone()).with_is_hidden(true);
                    self.modality.apply_possibility(factory, node, &non_q_and_p, extras)
                }
                else
                {
                    let non_q = Non(bx!(q.clone()), extras.clone());
                    self.modality.apply_possibility(factory, node, &non_q, extras)
                };

                factory.modality_graph.set_log_line_formatter(default_log_line_formatter!());

                let previous_world = extras.possible_world;
                let current_world = *factory.modality_graph.nodes.iter().max().unwrap();
                let current_vertex = GraphVertex::new(previous_world, current_world);
                factory.modality_graph.vertices_tags.push((current_vertex.clone(), p_as_string));

                let inherited_tags = factory.modality_graph.vertices_tags.iter()
                    .filter(|(vertex, _tag)| vertex.to == previous_world)
                    .map(|(_vertex, tag)| tag.clone()).collect::<Vec<String>>();
                for inherited_tag in inherited_tags
                {
                    factory.modality_graph.vertices_tags.push((current_vertex.clone(), inherited_tag));
                }

                return subtree;
            }

            Conditional(box p, box q, extras) =>
            {
                let mut formula_format_options = FormulaFormatOptions::default();
                formula_format_options.should_show_possible_worlds = false;
                let p_as_string = p.to_string_with_options(&formula_format_options);

                let reflexive_vertices = factory.modality_graph.vertices.iter()
                    .filter(|vertex| vertex.from == vertex.to)
                    .map(|vertex| vertex.clone())
                    .collect::<BTreeSet<GraphVertex>>();

                let vertices_with_right_tag = factory.modality_graph.vertices_tags.iter()
                    .filter(|(_vertex, tag)| *tag == p_as_string)
                    .map(|(vertex, _tag)| vertex.clone())
                    .collect::<BTreeSet<GraphVertex>>();

                let old_graph_vertices = factory.modality_graph.vertices.clone();
                let mut new_graph_vertices = reflexive_vertices;
                new_graph_vertices.extend(vertices_with_right_tag);
                factory.modality_graph.vertices = new_graph_vertices;

                let subtree = self.modality.apply_necessity(factory, node, &q, extras);

                //necessity reapplication not working properly in C
                factory.modality_graph.necessity_reapplications.clear();
                factory.modality_graph.vertices = old_graph_vertices;

                return subtree;
            }

            _ => None
        }
    }
}
