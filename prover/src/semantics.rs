pub mod binary_logic_semantics;
pub mod many_valued_logic_semantics;
pub mod fuzzy_logic_semantics;

use crate::formula::Formula;
use crate::logic::Logic;
use crate::tree::path::ProofTreePath;

pub trait Semantics
{
    fn number_of_truth_values(&self) -> u8;

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula;

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool;
}
