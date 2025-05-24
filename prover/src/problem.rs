pub mod catalog;
pub mod json;

use std::collections::BTreeSet;
use std::rc::Rc;
use crate::formula::{Formula, PredicateArgument, PredicateArguments};
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
    pub should_skip_contradiction_check : bool,
    pub non_rigid_designators : BTreeSet<PredicateArgument>,
}

impl Default for ProblemFlags
{
    fn default() -> Self
    {
        return ProblemFlags
        {
            should_skip_contradiction_check: false,
            non_rigid_designators: BTreeSet::new(),
        };
    }
}

impl Problem
{
    pub fn prove(self) -> ProofTree
    {
        let algorithm = ProofAlgorithm::initialize(self);
        let proof_tree = algorithm.prove();
        return proof_tree;
    }
}
