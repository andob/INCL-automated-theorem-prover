use std::any::Any;
use box_macro::bx;
use crate::formula::Formula::{And, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::ModalityRef;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::ManyValuedLogicSemantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub mod lukasiewicz_modal_logic;
pub mod rmingle3_modal_logic;
pub mod priest_logic_of_paradox;
pub mod kleene_modal_logic;
pub mod logic_with_gaps_and_gluts;
pub mod logic_of_constructible_negation;
mod generic_biimply_fde_rule;

//check out book chapters 8 and 22
pub struct MinimalFirstDegreeEntailmentLogic {}

impl Logic for MinimalFirstDegreeEntailmentLogic
{
    fn get_name(&self) -> LogicName { LogicName::of("MinimalFirstDegreeEntailmentLogic") }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(ManyValuedLogicSemantics::with_three_values());
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {})
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef> { None }
}

pub struct FirstDegreeEntailmentLogicRules {}
impl LogicRule for FirstDegreeEntailmentLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            And(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node_with_subnode(plus_p, plus_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            And(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node(minus_p);

                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, minus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node(plus_p);

                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                return Some(ProofSubtree::with_left_right_nodes(plus_p_node, plus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node_with_subnode(minus_p, minus_q_node);

                return Some(ProofSubtree::with_middle_node(minus_p_node));
            }

            Non(box Or(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);

                let non_p_and_non_q = And(bx!(non_p), bx!(non_q), extras.clone()).with_sign(extras.sign);
                let non_p_and_non_q_node = factory.new_node(non_p_and_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_and_non_q_node));
            }

            Non(box And(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);

                let non_p_or_non_q = Or(bx!(non_p), bx!(non_q), extras.clone()).with_sign(extras.sign);
                let non_p_or_non_q_node = factory.new_node(non_p_or_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_or_non_q_node));
            }

            Non(box Non(box p, _), extras) =>
            {
                let signed_p = p.with_sign(extras.sign);
                let signed_p_node = factory.new_node(signed_p.clone());

                return Some(ProofSubtree::with_middle_node(signed_p_node));
            }

            _ => None
        }
    }
}
