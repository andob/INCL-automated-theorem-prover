pub mod binary_semantics;

use crate::formula::Formula;

pub trait Semantics
{
    fn are_formulas_contradictory(&self, left : &Formula, right : &Formula) -> bool;
}
