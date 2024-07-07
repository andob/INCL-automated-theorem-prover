pub mod binary_logic_semantics;
pub mod three_valued_logic_semantics;

use crate::formula::Formula;

pub trait Semantics
{
    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula;

    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool;
}
