use box_macro::bx;
use crate::formula::Formula::{Exists, ForAll, Non};
use crate::logic::{Logic, LogicRule};
use crate::logic::propositional_logic::{BasicRules, DoubleNegationRule};
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_semantics::BinarySemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::subtree::ProofSubtree;

pub struct FirstOrderLogic {}
impl Logic for FirstOrderLogic
{
    fn get_name(&self) -> &str
    {
        return "FirstOrderLogic";
    }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinarySemantics{});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithArgs,
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Exists, TokenTypeID::ForAll,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(BasicRules {}),
            Box::new(DoubleNegationRule {}),
            Box::new(QuantifierNegationRules {}),
            Box::new(ExistsRule {}),
            Box::new(ForAllRule {}),
        ];
    }
}

struct QuantifierNegationRules {}
impl LogicRule for QuantifierNegationRules
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let Non(box Exists(x, box p, _), extras) = &node.formula
        {
            let non_p = Non(bx!(p.with(extras)), extras.clone());
            let for_all_non_p = ForAll(x.clone(), bx!(non_p), extras.clone());
            let for_all_non_p_node = factory.new_node(for_all_non_p);
            return Some(ProofSubtree::with_middle_node(for_all_non_p_node));
        }
        else if let Non(box ForAll(x, box p, _), extras) = &node.formula
        {
            let non_p = Non(bx!(p.with(extras)), extras.clone());
            let exists_non_p = Exists(x.clone(), bx!(non_p), extras.clone());
            let exists_non_p_node = factory.new_node(exists_non_p);
            return Some(ProofSubtree::with_middle_node(exists_non_p_node));
        }

        return None;
    }
}

struct ExistsRule {}
impl LogicRule for ExistsRule
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let Exists(x, box p, extras) = &node.formula
        {
            let instantiated_p = p.instantiated(factory, x, extras);
            let instantiated_p_node = factory.new_node(instantiated_p);
            return Some(ProofSubtree::with_middle_node(instantiated_p_node));
        }

        return None;
    }
}

struct ForAllRule {}
impl LogicRule for ForAllRule
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let ForAll(x, box p, extras) = &node.formula
        {
            //todo implement this
            let instantiated_p = p.instantiated(factory, x, extras);
            let instantiated_p_node = factory.new_node(instantiated_p);
            return Some(ProofSubtree::with_middle_node(instantiated_p_node));
        }

        return None;
    }
}
