use std::fmt::{Display, Formatter};
use crate::formula::Formula;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::path::ProofTreePath;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ProofTreeNode
{
    pub id : ProofTreeNodeID,
    pub formula : Formula,
    pub left : Option<Box<ProofTreeNode>>,
    pub middle : Option<Box<ProofTreeNode>>,
    pub right : Option<Box<ProofTreeNode>>,
    pub spawner_node_id : Option<ProofTreeNodeID>,
    pub contrarian_node_id: Option<ProofTreeNodeID>,
    pub is_contradictory : bool,
}

impl ProofTreeNode
{
    pub fn get_all_paths(&self) -> Vec<ProofTreePath>
    {
        let root_node = self;
        let mut paths : Vec<ProofTreePath> = vec![];
        let path = ProofTreePath::new(&root_node);
        root_node.find_all_paths(&mut paths, path);
        return paths;
    }

    fn find_all_paths(&self, out_paths : &mut Vec<ProofTreePath>, path : ProofTreePath)
    {
        if self.left.is_none() && self.middle.is_none() && self.right.is_none()
        {
            out_paths.push(path);
        }
        else
        {
            if let Some(left) = &self.left
            {
                left.find_all_paths(out_paths, path.plus(left));
            }

            if let Some(middle) = &self.middle
            {
                middle.find_all_paths(out_paths, path.plus(middle));
            }

            if let Some(right) = &self.right
            {
                right.find_all_paths(out_paths, path.plus(right));
            }
        }
    }

    pub fn get_node_with_id(&self, node_id : ProofTreeNodeID) -> Option<&ProofTreeNode>
    {
        if let Some(left) = &self.left
        {
            if let r@Some(_) = left.get_node_with_id(node_id) { return r };
        }

        if let Some(middle) = &self.middle
        {
            if let r@Some(_) = middle.get_node_with_id(node_id) { return r };
        }

        if let Some(right) = &self.right
        {
            if let r@Some(_) = right.get_node_with_id(node_id) { return r };
        }

        return if self.id == node_id { Some(&self) } else { None };
    }

    pub fn mark_child_node_as_contradictory(&mut self, node_id : ProofTreeNodeID, contrarian_node_id : ProofTreeNodeID)
    {
        if let Some(left) = &mut self.left
        {
            left.mark_child_node_as_contradictory(node_id, contrarian_node_id);
        }

        if let Some(middle) = &mut self.middle
        {
            middle.mark_child_node_as_contradictory(node_id, contrarian_node_id);
        }

        if let Some(right) = &mut self.right
        {
            right.mark_child_node_as_contradictory(node_id, contrarian_node_id);
        }

        if self.id == node_id
        {
            self.is_contradictory = true;
            self.contrarian_node_id = Some(contrarian_node_id);
        }
    }
}

impl Display for ProofTreeNode
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.formula)
    }
}
