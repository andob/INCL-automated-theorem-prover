use std::any::Any;
use std::collections::BTreeSet;
use box_macro::bx;
use crate::default_log_line_formatter;
use crate::formula::{AtomicFormulaExtras, Formula, PossibleWorld, PredicateArguments};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, DefinitelyExists, Equals, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::to_string::FormulaFormatOptions;
use crate::graph::GraphVertex;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalityRef};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::path::ProofTreePath;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 5 and 19
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
        return if !self.is_extended { LogicName::of("ConditionalModalLogic") }
        else { LogicName::of("ConditionalExtModalLogic") };
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

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
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
                let logic = logic_pointer.cast_to::<ConditionalModalLogic>()?;

                let mut formula_format_options = FormulaFormatOptions::default();
                formula_format_options.should_show_possible_worlds = false;
                let p_as_string = p.to_string_with_options(&formula_format_options);

                factory.modality_graph.set_log_line_formatter(bx!(move |v|
                    format!("{}R{} [{}]\n", v.from, v.to, p_as_string)));

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
                let current_world = *factory.modality_graph.nodes.iter().max()?;
                let current_vertex = GraphVertex::new(previous_world, current_world);
                factory.modality_graph.vertices_tags.push((current_vertex.clone(), p.with_stripped_extras()));

                let inherited_tags = factory.modality_graph.vertices_tags.iter()
                    .filter(|(vertex, _tag)| vertex.to == previous_world)
                    .map(|(_vertex, tag)| tag.clone()).collect::<Vec<Formula>>();
                for inherited_tag in inherited_tags
                {
                    factory.modality_graph.vertices_tags.push((current_vertex.clone(), inherited_tag));
                }

                return subtree;
            }

            Conditional(box p, box q, extras) =>
            {
                let reflexive_vertices = factory.modality_graph.vertices.iter()
                    .filter(|vertex| vertex.from == vertex.to)
                    .map(|vertex| vertex.clone())
                    .collect::<BTreeSet<GraphVertex>>();

                let p_without_extras = p.with_stripped_extras();
                let paths = factory.tree.get_paths_that_goes_through_node(node);
                let vertices_with_right_tag = factory.modality_graph.vertices_tags.iter()
                    .filter(|(_vertex, tag)| p_without_extras.is_replaceable_with(tag, &paths))
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

impl Formula
{
    fn is_replaceable_with(&self, another : &Formula, paths : &Vec<ProofTreePath>) -> bool
    {
        return match self
        {
            Atomic(p_name, p_extras) =>
            {
                if let Atomic(q_name, q_extras) = another
                { p_name == q_name && p_extras.is_replaceable_with(q_extras, paths) } else { false }
            }
            Non(box p, p_extras) =>
            {
                if let Non(box q, q_extras) = another
                { p.is_replaceable_with(p, paths) && p_extras == q_extras } else { false }
            }
            And(box p, box q, pq_extras) =>
            {
                if let And(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            Or(box p, box q, pq_extras) =>
            {
                if let Or(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            Imply(box p, box q, pq_extras) =>
            {
                if let Imply(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            BiImply(box p, box q, pq_extras) =>
            {
                if let BiImply(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            StrictImply(box p, box q, pq_extras) =>
            {
                if let StrictImply(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            Conditional(box p, box q, pq_extras) =>
            {
                if let Conditional(box w, box e, we_extras) = another
                { p.is_replaceable_with(w, paths) && q.is_replaceable_with(e, paths) && pq_extras == we_extras } else { false }
            }
            Exists(x, box p, p_extras) =>
            {
                if let Exists(y, box q, q_extras) = another
                { x==y && p.is_replaceable_with(q, paths) && p_extras == q_extras } else { false }
            }
            ForAll(x, box p, p_extras) =>
            {
                if let ForAll(y, box q, q_extras) = another
                { x==y && p.is_replaceable_with(q, paths) && p_extras == q_extras } else { false }
            }
            Equals(x, y, p_extras) =>
            {
                if let Equals(z, t, q_extras) = another
                { ((x==z && y==t) || (x==t && y==z)) && p_extras == q_extras } else { false }
            }
            DefinitelyExists(x, p_extras) =>
            {
                if let DefinitelyExists(y, q_extras) = another
                { x==y && p_extras == q_extras } else { false }
            }
            Possible(box p, p_extras) =>
            {
                if let Possible(box q, q_extras) = another
                { p.is_replaceable_with(p, paths) && p_extras == q_extras } else { false }
            }
            Necessary(box p, p_extras) =>
            {
                if let Necessary(box q, q_extras) = another
                { p.is_replaceable_with(p, paths) && p_extras == q_extras } else { false }
            }
            InPast(box p, p_extras) =>
            {
                if let InPast(box q, q_extras) = another
                { p.is_replaceable_with(p, paths) && p_extras == q_extras } else { false }
            }
            InFuture(box p, p_extras) =>
            {
                if let InFuture(box q, q_extras) = another
                { p.is_replaceable_with(p, paths) && p_extras == q_extras } else { false }
            }
            Comment(payload) =>
            {
                if let Comment(another_payload) = another
                { another_payload == payload } else { false }
            }
        }
    }
}

impl AtomicFormulaExtras
{
    fn is_replaceable_with(&self, another : &AtomicFormulaExtras, paths : &Vec<ProofTreePath>) -> bool
    {
        return self.predicate_args.are_replaceable_with(&another.predicate_args, paths, another.possible_world) &&
                self.sign == another.sign && self.is_hidden == another.is_hidden &&
                self.possible_world == another.possible_world;
    }
}

impl PredicateArguments
{
    fn are_replaceable_with(&self, another : &PredicateArguments, paths : &Vec<ProofTreePath>, possible_world : PossibleWorld) -> bool
    {
        return paths.iter().any(|path| self.with_equivalences(path, possible_world) == another.with_equivalences(path, possible_world));
    }
}
