pub mod binary_semantics;

use crate::formula::{Formula, FormulaExtras};

pub trait Semantics
{
    fn negate(&self, p : &Formula, extras : &FormulaExtras) -> Formula;

    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool;
}
