use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::codeloc;
use crate::formula::notations::OperatorNotations;
use crate::problem::json::ProblemJSON;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::ProofTree;

#[derive(Serialize, Deserialize)]
struct ProofTreeJSON
{
    problem : ProblemJSON,
    was_proved : bool,
    has_timeout : bool,
    root_node : ProofTreeNodeJSON,
}

#[derive(Serialize, Deserialize)]
struct ProofTreeNodeJSON
{
    id : ProofTreeNodeID,
    formula : String,
    is_contradictory : bool,
    spawner_node_id : Option<ProofTreeNodeID>,
    contrarian_node_id : Option<ProofTreeNodeID>,
    left : Option<Box<ProofTreeNodeJSON>>,
    middle : Option<Box<ProofTreeNodeJSON>>,
    right : Option<Box<ProofTreeNodeJSON>>,
}

impl ProofTree
{
    pub fn to_json(&self, notations : OperatorNotations) -> Result<String>
    {
        let json = ProofTreeJSON
        {
            problem: self.problem.to_json(notations),
            was_proved: self.is_proof_correct,
            has_timeout: self.has_timeout,
            root_node: self.root_node.to_json(notations),
        };

        return serde_json::to_string_pretty(&json).context(codeloc!());
    }
}

impl ProofTreeNode
{
    pub fn to_json(&self, notations : OperatorNotations) -> ProofTreeNodeJSON
    {
        return ProofTreeNodeJSON
        {
            id: self.id,
            formula: self.formula.to_string_with_notations(notations),
            is_contradictory: self.is_contradictory,
            spawner_node_id: self.spawner_node_id,
            contrarian_node_id: self.contrarian_node_id,
            left: if let Some(left) = &self.left
                  { Some(Box::new(left.to_json(notations))) } else { None },
            middle: if let Some(middle) = &self.middle
                    { Some(Box::new(middle.to_json(notations))) } else { None },
            right: if let Some(right) = &self.right
                   { Some(Box::new(right.to_json(notations))) } else { None },
        };
    }
}
