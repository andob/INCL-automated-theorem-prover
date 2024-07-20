use crate::formula::Formula;
use crate::formula::Formula::Atomic;
use crate::formula::Sign::{Minus, Plus};
use crate::semantics::Semantics;

pub struct ThreeValuedLogicSemantics {}
impl Semantics for ThreeValuedLogicSemantics
{
    //P could be true or false or unknown
    fn number_of_truth_values(&self) -> u8 { 3 }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return formula.with_sign(Minus);
    }

    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool
    {
        //todo this does not account for predicate arguments

        return match (p, q)
        {
            (Atomic(p_name, p_extras), Atomic(q_name, q_extras))
                if (p_extras.sign == Minus && q_extras.sign == Plus) || (p_extras.sign == Plus && q_extras.sign == Minus)

            => {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }
}
