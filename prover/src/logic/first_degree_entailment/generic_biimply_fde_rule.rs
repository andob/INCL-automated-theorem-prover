use box_macro::bx;
use crate::formula::Formula::{And, BiImply, Imply, Non};
use crate::formula::{Formula, FormulaExtras};
use crate::formula::Sign::Plus;
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct GenericBiImplyAsConjunctionRule {}
impl GenericBiImplyAsConjunctionRule
{
    fn build_generic_biimply_conjunction(&self, p : &Formula, q : &Formula, extras : &FormulaExtras) -> Formula
    {
        let p = p.in_world(extras.possible_world);
        let q = q.in_world(extras.possible_world);

        let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone()).with_sign(Plus);
        let q_imply_p = Imply(bx!(q), bx!(p), extras.clone()).with_sign(Plus);

        return And(bx!(p_imply_q), bx!(q_imply_p), extras.with_sign(Plus).with_is_hidden(true));
    }
}

impl LogicRule for GenericBiImplyAsConjunctionRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            BiImply(box p, box q, extras) =>
            {
                let conjunction = self.build_generic_biimply_conjunction(p, q, extras);
                let conjunction_node = factory.new_node(conjunction);

                return Some(ProofSubtree::with_middle_node(conjunction_node));
            }

            Non(box BiImply(box p, box q, ..), extras) =>
            {
                let conjunction = self.build_generic_biimply_conjunction(p, q, extras);
                let non_conjunction = Non(bx!(conjunction), extras.with_sign(Plus).with_is_hidden(true));
                let non_conjunction_node = factory.new_node(non_conjunction);

                return Some(ProofSubtree::with_middle_node(non_conjunction_node));
            }

            _ => None
        }
    }
}
