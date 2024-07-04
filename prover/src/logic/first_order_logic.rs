use std::any::Any;
use box_macro::bx;
use crate::formula::Formula::{Exists, ForAll, Non};
use crate::logic::{Logic, LogicRule};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_semantics::BinarySemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct FirstOrderLogic {}
impl Logic for FirstOrderLogic
{
    fn get_name(&self) -> &str { "FirstOrderLogic" }
    fn as_any(&self) -> &dyn Any { self }

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
            Box::new(PropositionalLogicRules {}),
            Box::new(QuantifierRules {}),
        ];
    }
}

struct QuantifierRules {}
impl LogicRule for QuantifierRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let logic_semantics = factory.get_logic().get_semantics();

        return match &node.formula
        {
            Non(box Exists(x, box p, _), extras) =>
            {
                let non_p = logic_semantics.negate(p, extras);
                let for_all_non_p = ForAll(x.clone(), bx!(non_p), extras.clone());
                let for_all_non_p_node = factory.new_node(for_all_non_p);
                return Some(ProofSubtree::with_middle_node(for_all_non_p_node));
            }

            Non(box ForAll(x, box p, _), extras) =>
            {
                let non_p = logic_semantics.negate(p, extras);
                let exists_non_p = Exists(x.clone(), bx!(non_p), extras.clone());
                let exists_non_p_node = factory.new_node(exists_non_p);
                return Some(ProofSubtree::with_middle_node(exists_non_p_node));
            }

            Exists(x, box p, extras) =>
            {
                let instantiated_p = p.instantiated(factory, x, extras);
                let instantiated_p_node = factory.new_node(instantiated_p);
                return Some(ProofSubtree::with_middle_node(instantiated_p_node));
            }

            ForAll(x, box p, extras) =>
            {
                let instantiated_p = p.instantiated(factory, x, extras);
                let instantiated_p_node = factory.new_node(instantiated_p);
                return Some(ProofSubtree::with_middle_node(instantiated_p_node));
            }

            _ => None,
        }
    }
}