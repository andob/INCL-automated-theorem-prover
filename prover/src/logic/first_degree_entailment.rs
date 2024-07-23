use std::any::Any;
use box_macro::bx;
use crate::formula::Formula::{And, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::three_valued_logic_semantics::ThreeValuedLogicSemantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub mod lukasiewicz_modal_logic;
pub mod rmingle3_modal_logic;
pub mod priest_logic_of_paradox;
pub mod kleene_modal_logic;

//check out book chapter 8
pub struct MinimalFirstDegreeEntailmentLogic {}

impl Logic for MinimalFirstDegreeEntailmentLogic
{
    fn get_name(&self) -> LogicName { LogicName::MinimalFirstDegreeEntailmentLogic }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(ThreeValuedLogicSemantics::new());
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

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {})
        ]
    }
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
                let q = q.in_world(extras.possible_world);
                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                let p = p.in_world(extras.possible_world);
                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node_with_subnode(plus_p, plus_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            And(box p, box q, extras) if extras.sign == Minus =>
            {
                let p = p.in_world(extras.possible_world);
                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node(minus_p);

                let q = q.in_world(extras.possible_world);
                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, minus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Plus =>
            {
                let p = p.in_world(extras.possible_world);
                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node(plus_p);

                let q = q.in_world(extras.possible_world);
                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                return Some(ProofSubtree::with_left_right_nodes(plus_p_node, plus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Minus =>
            {
                let q = q.in_world(extras.possible_world);
                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                let p = p.in_world(extras.possible_world);
                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node_with_subnode(minus_p, minus_q_node);

                return Some(ProofSubtree::with_middle_node(minus_p_node));
            }

            Non(box Or(box p, box q, _), extras) =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);
                let non_p = Non(bx!(p), extras.clone()).with_sign(Plus);
                let non_q = Non(bx!(q), extras.clone()).with_sign(Plus);

                let non_p_and_non_q = And(bx!(non_p), bx!(non_q), extras.clone()).with_sign(extras.sign);
                let non_p_and_non_q_node = factory.new_node(non_p_and_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_and_non_q_node));
            }

            Non(box And(box p, box q, _), extras) =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);
                let non_p = Non(bx!(p), extras.clone()).with_sign(Plus);
                let non_q = Non(bx!(q), extras.clone()).with_sign(Plus);

                let non_p_or_non_q = Or(bx!(non_p), bx!(non_q), extras.clone()).with_sign(extras.sign);
                let non_p_or_non_q_node = factory.new_node(non_p_or_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_or_non_q_node));
            }

            Non(box Non(box p, _), extras) =>
            {
                let p = p.in_world(extras.possible_world).with_sign(extras.sign);
                let p_node = factory.new_node(p);

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            _ => None
        }
    }
}
