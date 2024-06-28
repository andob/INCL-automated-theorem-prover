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
    pub is_contradictory : bool,
}

//todo migrate to this data structure. Use less object cloning.
// pub struct ProofTreeNode
// {
//     data : Rc<RefCell<ProofTreeNodeData>>
// }
//
// pub struct ProofTreeNodeData
// {
//     pub id : ProofTreeNodeID,
//     pub formula : Formula,
//     pub left : Option<Box<ProofTreeNode>>,
//     pub middle : Option<Box<ProofTreeNode>>,
//     pub right : Option<Box<ProofTreeNode>>,
//     pub is_contradictory : bool,
// }
//
// impl Deref for ProofTreeNode
// {
//     type Target = ProofTreeNodeData;
//     fn deref(&self) -> &Self::Target { &*self.data.borrow() }
// }
//
// impl DerefMut for ProofTreeNode
// {
//     fn deref_mut(&mut self) -> &mut Self::Target { &mut *self.data.borrow_mut() }
// }

impl ProofTreeNode
{
    pub fn get_all_leafs(&self) -> Vec<ProofTreeNode>
    {
        let (leafs, _paths) = self.get_all_leafs_with_paths();
        return leafs;
    }

    pub fn get_all_paths(&self) -> Vec<ProofTreePath>
    {
        let (_leafs, paths) = self.get_all_leafs_with_paths();
        return paths;
    }

    pub fn get_all_leafs_with_paths(&self) -> (Vec<ProofTreeNode>, Vec<ProofTreePath>)
    {
        let root_node = self;
        let mut leafs : Vec<ProofTreeNode> = vec![];
        let mut paths : Vec<ProofTreePath> = vec![];
        let path = ProofTreePath::new(vec![root_node.clone()]);
        root_node.find_all_leafs_with_paths(&mut leafs, &mut paths, path);
        return (leafs, paths);
    }

    fn find_all_leafs_with_paths(&self, out_leafs : &mut Vec<ProofTreeNode>, out_paths : &mut Vec<ProofTreePath>, path : ProofTreePath)
    {
        if self.left.is_none() && self.middle.is_none() && self.right.is_none()
        {
            out_leafs.push(self.clone());
            out_paths.push(path);
        }
        else
        {
            if let Some(left) = &self.left
            {
                left.find_all_leafs_with_paths(out_leafs, out_paths, path.plus(left));
            }

            if let Some(middle) = &self.middle
            {
                middle.find_all_leafs_with_paths(out_leafs, out_paths, path.plus(middle));
            }

            if let Some(right) = &self.right
            {
                right.find_all_leafs_with_paths(out_leafs, out_paths, path.plus(right));
            }
        }
    }

    //todo use this
    pub fn get_all_child_nodes(&self) -> Vec<ProofTreeNode>
    {
        let mut nodes: Vec<ProofTreeNode> = vec![];
        self.find_all_child_nodes(&mut nodes);
        return nodes;
    }

    fn find_all_child_nodes(&self, out_nodes : &mut Vec<ProofTreeNode>)
    {
        out_nodes.push(self.clone());

        if let Some(left) = &self.left
        {
            left.find_all_child_nodes(out_nodes);
        }

        if let Some(middle) = &self.middle
        {
            middle.find_all_child_nodes(out_nodes);
        }

        if let Some(right) = &self.right
        {
            right.find_all_child_nodes(out_nodes);
        }
    }

    //todo remove this stupidity after migration to Rc<RefCell<>>
    pub fn mark_child_node_as_contradictory(&mut self, node_id : ProofTreeNodeID)
    {
        if let Some(left) = &mut self.left
        {
            left.mark_child_node_as_contradictory(node_id);
        }

        if let Some(middle) = &mut self.middle
        {
            middle.mark_child_node_as_contradictory(node_id);
        }

        if let Some(right) = &mut self.right
        {
            right.mark_child_node_as_contradictory(node_id);
        }

        if self.id == node_id
        {
            self.is_contradictory = true;
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
