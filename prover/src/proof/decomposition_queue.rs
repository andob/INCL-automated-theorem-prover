use crate::formula::Formula;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

#[derive(Eq, PartialEq, Hash)]
enum Priority { High, Normal, Low }

pub struct DecompositionPriorityQueue
{
    priorities : Vec<Priority>,
    consumable_nodes : Vec<Box<ProofTreeNode>>,
}

impl DecompositionPriorityQueue
{
    pub fn new() -> DecompositionPriorityQueue
    {
        return DecompositionPriorityQueue
        {
            priorities: vec![Priority::High, Priority::Normal, Priority::Low],
            consumable_nodes: vec![],
        };
    }

    pub fn is_empty(&self) -> bool
    {
        return self.consumable_nodes.is_empty();
    }

    pub fn push_subtree(&mut self, subtree : Box<ProofSubtree>)
    {
        if let Some(left) = subtree.left { self.push_tree_node(left); }
        if let Some(middle) = subtree.middle { self.push_tree_node(middle); }
        if let Some(right) = subtree.right { self.push_tree_node(right); }
    }

    pub fn push_tree_node(&mut self, node : Box<ProofTreeNode>)
    {
        if let Some(left) = &node.left { self.push_tree_node(left.clone()); }
        if let Some(middle) = &node.middle { self.push_tree_node(middle.clone()); }
        if let Some(right) = &node.right { self.push_tree_node(right.clone()); }
        self.consumable_nodes.push(node);
    }

    pub fn pop(&mut self) -> Option<Box<ProofTreeNode>>
    {
        if self.consumable_nodes.is_empty()
        {
            return None;
        }

        for priority in &self.priorities
        {
            for index in 0..self.consumable_nodes.len()
            {
                if self.get_node_priority(&self.consumable_nodes[index]) == *priority
                {
                    return Some(self.consumable_nodes.remove(index));
                }
            }
        }

        return None;
    }

    fn get_node_priority(&self, node : &Box<ProofTreeNode>) -> Priority
    {
        return match &node.formula
        {
            Formula::ForAll(..) => Priority::Low,
            _ => Priority::High,
        }
    }
}
