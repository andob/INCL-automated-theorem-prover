pub mod catalog;
pub mod json;

use std::rc::Rc;
use crate::formula::Formula;
use crate::logic::Logic;
use crate::proof::ProofAlgorithm;
use crate::tree::ProofTree;

#[derive(Clone)]
pub struct Problem
{
    pub id : String,
    pub logic : Rc<dyn Logic>,
    pub premises : Vec<Formula>,
    pub conclusion : Formula,
    pub flags : ProblemFlags,
}

#[derive(Clone)]
pub struct ProblemFlags
{
    pub should_skip_contradiction_check : bool
}

impl Default for ProblemFlags
{
    fn default() -> Self
    {
        return ProblemFlags { should_skip_contradiction_check: false };
    }
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
