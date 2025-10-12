use std::fmt::{Display, Formatter};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use crate::formula::to_string::FormulaFormatOptions;
use crate::logic::{LogicRuleResult, LogicRuleResultCollection};
use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::ProofTree;

impl LogicRuleResult
{
    pub fn is_empty(&self) -> bool
    {
        match self
        {
            LogicRuleResult::Empty => true,
            LogicRuleResult::Subtree(subtree) => subtree.is_empty(),
            LogicRuleResult::Subtrees(subtrees) =>
                subtrees.iter().all(|(_, subtree)| subtree.is_empty()),
            LogicRuleResult::FromMultipleResults(results) =>
                results.iter().all(|result| result.is_empty())
        }
    }

    pub fn to_string_with_options(&self, options : &FormulaFormatOptions) -> String
    {
        if self.is_empty() { return String::from("Empty result") }
        let mut output_string = String::new();

        match self
        {
            LogicRuleResult::Empty => {}
            LogicRuleResult::Subtree(subtree) =>
            {
                let subtree_as_string = subtree.to_string_with_options(options);
                output_string.push_str(subtree_as_string.as_str());
            }

            LogicRuleResult::Subtrees(subtrees) =>
            {
                for (leaf_node_id, subtree) in subtrees
                {
                    let subtree_as_string = subtree.to_string_with_options(options);
                    let description = format!("\nOn branch with leaf ID {}:\n{}", leaf_node_id, subtree_as_string);
                    output_string.push_str(description.as_str());
                }
            }

            LogicRuleResult::FromMultipleResults(results) =>
            {
                output_string.push_str("\nMultiple results:");
                for (index, result) in results.iter().enumerate()
                {
                    let result_as_string = result.to_string_with_options(options);
                    let description = format!("\n{}: {}", index, result_as_string);
                    output_string.push_str(description.as_str());
                }
            }
        }

        return output_string;
    }

    pub fn hide_all_nodes(&mut self)
    {
        match self
        {
            LogicRuleResult::Empty => {}
            LogicRuleResult::Subtree(subtree) =>
            {
                subtree.hide_all_nodes();
            }

            LogicRuleResult::Subtrees(subtrees) =>
            {
                for (_, subtree) in subtrees
                {
                    subtree.hide_all_nodes();
                }
            }

            LogicRuleResult::FromMultipleResults(results) =>
            {
                for result in results.iter_mut()
                {
                    result.hide_all_nodes();
                }
            }
        }
    }
}

impl Display for LogicRuleResult
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let options = FormulaFormatOptions::default();
        return write!(f, "{}", self.to_string_with_options(&options));
    }
}

impl ProofTree
{
    pub fn append_logic_rule_result(&mut self, result : &mut LogicRuleResult, target_node_id : ProofTreeNodeID)
    {
        match result
        {
            LogicRuleResult::Empty => {}
            LogicRuleResult::Subtree(subtree) =>
            {
                self.append_subtree(subtree, target_node_id);
            }

            LogicRuleResult::Subtrees(subtrees) =>
            {
                for (leaf_node_id, subtree) in subtrees
                {
                    self.append_subtree(subtree, *leaf_node_id);
                }
            }

            LogicRuleResult::FromMultipleResults(results) =>
            {
                for result in results.iter_mut()
                {
                    self.append_logic_rule_result(result, target_node_id);
                }
            }
        }
    }
}

impl DecompositionPriorityQueue
{
    pub fn push_logic_rule_result(&mut self, result : LogicRuleResult)
    {
        match result
        {
            LogicRuleResult::Empty => {}
            LogicRuleResult::Subtree(subtree) =>
            {
                self.push_subtree(Box::new(subtree));
            }

            LogicRuleResult::Subtrees(subtrees) =>
            {
                for (_, mut subtree) in subtrees
                {
                    //todo is this necessary?
                    subtree.cloned_subtrees_with_new_ids.clear();
                    self.push_subtree(Box::new(subtree));
                }
            }

            LogicRuleResult::FromMultipleResults(results) =>
            {
                for child_result in results.into_iter()
                {
                    self.push_logic_rule_result(child_result);
                }
            }
        }
    }
}

impl LogicRuleResultCollection
{
    pub fn new() -> LogicRuleResultCollection
    {
        return LogicRuleResultCollection { results:Vec::new() };
    }

    pub fn with(result : LogicRuleResult) -> LogicRuleResultCollection
    {
        return LogicRuleResultCollection { results:vec![result] };
    }

    pub fn push(&mut self, result : LogicRuleResult)
    {
        if !result.is_empty()
        {
            self.results.push(result);
        }
    }

    pub fn iter(&self) -> Iter<'_, LogicRuleResult>
    {
        return self.results.iter();
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, LogicRuleResult>
    {
        return self.results.iter_mut();
    }

    pub fn into_iter(self) -> IntoIter<LogicRuleResult>
    {
        return self.results.into_iter();
    }

    pub fn joined(self) -> LogicRuleResult
    {
        if !self.results.is_empty()
        {
            return LogicRuleResult::FromMultipleResults(self);
        }

        return LogicRuleResult::Empty;
    }
}
