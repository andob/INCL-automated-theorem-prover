mod exists;
mod forall;
mod identity;
pub mod modal_logic;

use std::any::Any;
use std::collections::BTreeSet;
use box_macro::bx;
use crate::formula::Formula::{Atomic, Equals, Exists, ForAll, Non};
use crate::formula::PredicateArgument;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::first_order_logic::exists::apply_existential_quantification;
use crate::logic::first_order_logic::forall::{apply_for_all_quantification, generate_possible_atomic_contradictory_formulas};
use crate::logic::first_order_logic::identity::generate_missing_transitive_equalities;
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 12
pub struct FirstOrderLogic {}
impl Logic for FirstOrderLogic
{
    fn get_name(&self) -> LogicName { LogicName::FirstOrderLogic }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinaryLogicSemantics {});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithArgs,
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Exists, TokenTypeID::ForAll, TokenTypeID::Equals,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(QuantifierRules {}),
        ]
    }
}

struct QuantifierRules {}
impl LogicRule for QuantifierRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Exists(x, box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let for_all_non_p = ForAll(x.clone(), bx!(non_p), extras.clone());
                let for_all_non_p_node = factory.new_node(for_all_non_p);

                return Some(ProofSubtree::with_middle_node(for_all_non_p_node));
            }

            Non(box ForAll(x, box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let exists_non_p = Exists(x.clone(), bx!(non_p), extras.clone());
                let exists_non_p_node = factory.new_node(exists_non_p);

                return Some(ProofSubtree::with_middle_node(exists_non_p_node));
            }

            Exists(x, box p, extras) =>
            {
                return apply_existential_quantification(factory, node, x, p, extras);
            }

            ForAll(x, box p, extras) =>
            {
                return apply_for_all_quantification(factory, node, x, p, extras);
            }

            Non(box Equals(x, y, _), extras) =>
            {
                if x == y
                {
                    //this is an object that is not equal to self ~(x=x)? forcing contradiction by stating x=x.
                    let x_equals_x = Equals(x.clone(), x.clone(), extras.clone());
                    let x_equals_x_node = factory.new_node(x_equals_x);

                    return Some(ProofSubtree::with_middle_node(x_equals_x_node));
                }

                return None;
            }

            Equals(_, _, extras) =>
            {
                //foreach node pair (x = y, y = z), generate a transitive node x = z
                let equalities = factory.tree.get_paths_that_goes_through_node(node).into_iter()
                    .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
                    .filter(|formula| formula.get_possible_world() == extras.possible_world)
                    .filter_map(|formula| if let Equals(x, y, _)
                        = formula { Some((x.clone(), y.clone())) } else { None })
                    .collect::<BTreeSet<(PredicateArgument, PredicateArgument)>>();

                let nodes = generate_missing_transitive_equalities(equalities, extras).into_iter()
                    .map(|formula| factory.new_node(formula))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            Non(box p@Atomic(..), extras) =>
            {
                //foreach node pair (~P[b:x], b:x = a), generate a possible contradictory node ~P[a]
                let nodes = generate_possible_atomic_contradictory_formulas(factory, node, p, extras).into_iter()
                    .map(|formula| factory.new_node(formula))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            p@Atomic(_, extras) =>
            {
                //foreach node pair (P[b:x], b:x = a), generate a possible contradictory node ~[a]
                let nodes = generate_possible_atomic_contradictory_formulas(factory, node, p, &extras.to_formula_extras()).into_iter()
                    .map(|formula| factory.new_node(Non(Box::new(formula), extras.to_formula_extras())))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            _ => None,
        }
    }
}
