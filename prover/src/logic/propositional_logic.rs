use crate::formula::Formula::{And, BiImply, Imply, Non, Or};
use crate::logic::{Logic, LogicRule};
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_semantics::BinarySemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::subtree::ProofSubtree;

pub struct PropositionalLogic {}
impl Logic for PropositionalLogic
{
    fn get_name(&self) -> &str { "PropositionalLogic" }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinarySemantics{});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(DoubleNegationRule {}),
            Box::new(BasicNonSplittingRules {}),
            Box::new(BasicSplittingRules {}),
        ];
    }
}

pub struct DoubleNegationRule {}
impl LogicRule for DoubleNegationRule
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let Non(box Non(box p)) = &node.formula
        {
            let p_node = factory.new_node(p.clone());
            return Some(ProofSubtree::with_middle_node(p_node));
        }

        return None;
    }
}

pub struct BasicNonSplittingRules {}
impl LogicRule for BasicNonSplittingRules
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            And(box p, box q) =>
            {
                let q_node = factory.new_node(q.clone());
                let p_node = factory.new_node_with_subnode(p.clone(), q_node);
                return Some(ProofSubtree::with_middle_node(p_node));
            }

            Non(box Or(box p, box q)) =>
            {
                let non_q_node = factory.new_node(Non(q.to_box()));
                let non_p_node = factory.new_node_with_subnode(Non(p.to_box()), non_q_node);
                return Some(ProofSubtree::with_middle_node(non_p_node));
            }

            Non(box Imply(box p, box q)) =>
            {
                let non_q_node = factory.new_node(Non(q.to_box()));
                let p_node = factory.new_node_with_subnode(p.clone(), non_q_node);
                return Some(ProofSubtree::with_middle_node(p_node));
            }

            _ => None
        }
    }
}

pub struct BasicSplittingRules {}
impl LogicRule for BasicSplittingRules
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Or(box p, box q) =>
            {
                let p_node = factory.new_node(p.clone());
                let q_node = factory.new_node(q.clone());
                return Some(ProofSubtree::with_left_right_nodes(p_node, q_node));
            }

            Non(box And(box p, box q)) =>
            {
                let non_p_node = factory.new_node(Non(p.to_box()));
                let non_q_node = factory.new_node(Non(q.to_box()));
                return Some(ProofSubtree::with_left_right_nodes(non_p_node, non_q_node));
            }

            Imply(box p, box q) =>
            {
                let non_p_node = factory.new_node(Non(p.to_box()));
                let q_node = factory.new_node(q.clone());
                return Some(ProofSubtree::with_left_right_nodes(non_p_node, q_node));
            }

            BiImply(box p, box q) =>
            {
                let q_node = factory.new_node(q.clone());
                let non_q_node = factory.new_node(Non(q.to_box()));
                let p_node = factory.new_node_with_subnode(p.clone(), q_node);
                let non_p_node = factory.new_node_with_subnode(Non(p.to_box()), non_q_node);
                return Some(ProofSubtree::with_left_right_nodes(p_node, non_p_node));
            }

            Non(box BiImply(box p, box q)) =>
            {
                let q_node = factory.new_node(q.clone());
                let non_q_node = factory.new_node(Non(q.to_box()));
                let p_node = factory.new_node_with_subnode(p.clone(), non_q_node);
                let non_p_node = factory.new_node_with_subnode(Non(p.to_box()), q_node);
                return Some(ProofSubtree::with_left_right_nodes(p_node, non_p_node));
            }

            _ => None
        }
    }
}
