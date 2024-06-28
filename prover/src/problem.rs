pub mod catalog;
pub mod json;

use crate::formula::Formula;
use crate::logic::Logic;
use crate::proof::ProofAlgorithm;
use crate::tree::ProofTree;

pub struct Problem
{
    pub id : String,
    pub logic : Box<dyn Logic>,
    pub premises : Vec<Formula>,
    pub conclusion : Formula,
}

impl Problem
{
    pub fn prove(self) -> ProofTree
    {
        let mut algorithm = ProofAlgorithm::initialize(self);
        let proof_tree = algorithm.prove();
        return proof_tree;
    }
}
