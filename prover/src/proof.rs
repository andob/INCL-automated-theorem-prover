pub mod problem_catalog;
pub mod problem_json;

use crate::formula::Formula;

pub struct Problem
{
    pub premises : Vec<Formula>,
    pub conclusion : Formula,
}
