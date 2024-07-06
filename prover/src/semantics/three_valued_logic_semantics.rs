use std::ops::Not;
use crate::formula::{Formula, FormulaExtras, Sign};
use crate::semantics::Semantics;

pub struct ThreeValuedLogicSemantics {}
impl Semantics for ThreeValuedLogicSemantics
{
    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool
    {
        //todo this does not account for semantic tags
        //todo this does not account for possible worlds
        //todo this does not account for predicate arguments

        return false
    }
}
