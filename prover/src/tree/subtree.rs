use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::ProofTree;

pub struct ProofSubtree
{
    pub left : Option<Box<ProofTreeNode>>,
    pub middle : Option<Box<ProofTreeNode>>,
    pub right : Option<Box<ProofTreeNode>>,
}

impl ProofSubtree
{
    fn empty() -> ProofSubtree
    {
        return ProofSubtree { left:None, middle:None, right:None };
    }
}

impl<'a> ProofTree<'a>
{
    //todo test this
    pub fn append_subtree(&mut self, subtree : ProofSubtree, node_id : ProofTreeNodeID)
    {
        let mut target_leaf_node_ids : Vec<ProofTreeNodeID> = vec![];

        let (leafs, paths) = self.get_all_leafs_with_paths();
        for (index, leaf) in leafs.iter().enumerate()
        {
            let path = &paths[index];
            if !leaf.is_contradictory && (leaf.id == node_id || path.nodes.iter().any(|node| node.id == node_id))
            {
                target_leaf_node_ids.push(leaf.id);
            }
        }

        self.root_node.append_subtree_recursive(&subtree, &target_leaf_node_ids);
    }
}

impl ProofTreeNode
{
    fn append_subtree_recursive(&mut self, subtree : &ProofSubtree, node_ids : &Vec<ProofTreeNodeID>)
    {
        if node_ids.contains(&self.id)
        {
            self.left = subtree.left.clone();
            self.middle = subtree.middle.clone();
            self.right = subtree.right.clone();
        }
        else
        {
            if let Some(left) = &mut self.left
            {
                left.append_subtree_recursive(subtree, node_ids);
            }

            if let Some(middle) = &mut self.middle
            {
                middle.append_subtree_recursive(subtree, node_ids);
            }

            if let Some(right) = &mut self.right
            {
                right.append_subtree_recursive(subtree, node_ids);
            }
        }
    }
}
