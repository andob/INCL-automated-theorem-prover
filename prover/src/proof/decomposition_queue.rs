use std::rc::Rc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::formula::Formula::{And, Atomic, BiImply, Conditional, Equals, ForAll, Imply, Non, Or, Possible, StrictImply};
use crate::logic::Logic;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;

#[derive(Eq, PartialEq, Hash, EnumIter)]
enum Priority
{
    MostImportant,
    MoreImportant,
    Important,
    Normal,
    Unimportant,
    MoreUnimportant,
    MostUnimportant,
}

pub struct DecompositionPriorityQueue
{
    logic : Rc<dyn Logic>,
    priorities : Vec<Priority>,
    consumable_nodes : Vec<Box<ProofTreeNode>>,
    reusable_nodes : Vec<Box<ProofTreeNode>>,
    banned_reusable_nodes : Vec<Box<ProofTreeNode>>,
    previously_queued_node_ids : Vec<ProofTreeNodeID>,
}

impl DecompositionPriorityQueue
{
    pub fn new(logic : Rc<dyn Logic>) -> DecompositionPriorityQueue
    {
        return DecompositionPriorityQueue
        {
            logic: logic,
            priorities: Priority::iter().collect(),
            consumable_nodes: vec![],
            reusable_nodes: vec![],
            banned_reusable_nodes: vec![],
            previously_queued_node_ids: vec![],
        };
    }

    pub fn is_empty(&self) -> bool
    {
        return self.consumable_nodes.is_empty() && self.reusable_nodes.is_empty();
    }

    pub fn push_tree_node(&mut self, node : Box<ProofTreeNode>)
    {
        if let Some(left) = &node.left { self.push_tree_node(left.clone()); }
        if let Some(middle) = &node.middle { self.push_tree_node(middle.clone()); }
        if let Some(right) = &node.right { self.push_tree_node(right.clone()); }

        if !self.previously_queued_node_ids.contains(&node.id)
        {
            self.previously_queued_node_ids.push(node.id);
            self.consumable_nodes.push(node);
        }
    }

    pub fn pop(&mut self) -> Option<Box<ProofTreeNode>>
    {
        if self.consumable_nodes.is_empty() && !self.reusable_nodes.is_empty()
        {
            let reusable_node = self.reusable_nodes.remove(0);
            if !self.banned_reusable_nodes.contains(&reusable_node)
            {
                self.consumable_nodes.push(reusable_node.clone());
                self.banned_reusable_nodes.push(reusable_node);
            }
        }

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
                    let consumed_node = self.consumable_nodes.remove(index);
                    
                    if self.should_node_be_reused(&consumed_node)
                    {
                        self.reusable_nodes.push(consumed_node.clone());
                    }
                    
                    return Some(consumed_node);
                }
            }
        }

        return None;
    }

    fn get_node_priority(&self, node : &Box<ProofTreeNode>) -> Priority
    {
        return match &node.formula
        {
            //atomics are the least important
            Atomic(..) => Priority::MostUnimportant,
            Non(box Atomic(..), ..) => Priority::MostUnimportant,

            //forall needs to be applied after all instantiations
            ForAll(..) => Priority::MoreUnimportant,

            //conditional needs to be applied after possibility
            Conditional(..) => Priority::Unimportant,

            //on non-normal modal logic, possibility needs to be applied after necessity
            Non(box StrictImply(..), ..) | Possible(..)
            if self.logic.get_name().is_non_normal_modal_logic() => Priority::Normal,

            //propositional operations that will split the tree
            BiImply(..) | Non(box BiImply(..), ..) => Priority::Important,
            Or(..) | Non(box And(..), ..) | Imply(..) => Priority::Important,

            //equals and non-equals are most important
            Equals(..) | Non(box Equals(..), ..) => Priority::MostImportant,

            _ => Priority::MoreImportant,
        }
    }

    fn should_node_be_reused(&self, node : &Box<ProofTreeNode>) -> bool
    {
        return matches!(&node.formula, ForAll(..)) &&
            !self.banned_reusable_nodes.contains(node);
    }
}
