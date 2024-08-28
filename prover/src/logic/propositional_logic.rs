use std::any::Any;
use box_macro::bx;
use crate::formula::Formula::{And, BiImply, Imply, Non, Or};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::ModalityRef;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 1
pub struct PropositionalLogic {}
impl Logic for PropositionalLogic
{
    fn get_name(&self) -> LogicName { LogicName::of("WithoutModality") }
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
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec!
        [
            Box::new(PropositionalLogicRules {}),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef> { None }
}

pub struct PropositionalLogicRules {}
impl LogicRule for PropositionalLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Non(box p, _), _) =>
            {
                let p_node = factory.new_node(p.clone());

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            And(box p, box q, _) =>
            {
                let q_node = factory.new_node(q.clone());
                let p_node = factory.new_node_with_subnode(p.clone(), q_node);

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            Non(box And(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_node = factory.new_node(non_p);

                let non_q = Non(bx!(q.clone()), extras.clone());
                let non_q_node = factory.new_node(non_q);

                return Some(ProofSubtree::with_left_right_nodes(non_p_node, non_q_node));
            }

            Or(box p, box q, _) =>
            {
                let p_node = factory.new_node(p.clone());
                let q_node = factory.new_node(q.clone());

                return Some(ProofSubtree::with_left_right_nodes(p_node, q_node));
            }

            Non(box Or(box p, box q, _), extras) =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone());
                let non_q_node = factory.new_node(non_q);

                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_node = factory.new_node_with_subnode(non_p, non_q_node);

                return Some(ProofSubtree::with_middle_node(non_p_node));
            }

            Imply(box p, box q, extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_node = factory.new_node(non_p);

                let q_node = factory.new_node(q.clone());

                return Some(ProofSubtree::with_left_right_nodes(non_p_node, q_node));
            }

            Non(box Imply(box p, box q, _), extras) =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone());
                let non_q_node = factory.new_node(non_q);

                let p_node = factory.new_node_with_subnode(p.clone(), non_q_node);

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            BiImply(box p, box q, extras) =>
            {
                let q_node = factory.new_node(q.clone());
                let p_node = factory.new_node_with_subnode(p.clone(), q_node);

                let non_q = Non(bx!(q.clone()), extras.clone());
                let non_q_node = factory.new_node(non_q);

                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_node = factory.new_node_with_subnode(non_p, non_q_node);

                return Some(ProofSubtree::with_left_right_nodes(p_node, non_p_node));
            }

            Non(box BiImply(box p, box q, _), extras) =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone());
                let non_q_node = factory.new_node(non_q);

                let p_node = factory.new_node_with_subnode(p.clone(), non_q_node);
                let q_node = factory.new_node(q.clone());

                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_p_node = factory.new_node_with_subnode(non_p, q_node);

                return Some(ProofSubtree::with_left_right_nodes(p_node, non_p_node));
            }

            _ => None
        }
    }
}
