use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;
use crate::tree::ProofTree;

#[derive(Clone)]
pub struct ProofSubtree
{
    pub left : Option<Box<ProofTreeNode>>,
    pub middle : Option<Box<ProofTreeNode>>,
    pub right : Option<Box<ProofTreeNode>>,
}

impl ProofSubtree
{
    pub fn empty() -> ProofSubtree
    {
        return ProofSubtree { left:None, middle:None, right:None };
    }

    pub fn with_middle_node(node : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree { left:None, middle:Some(Box::new(node)), right:None };
    }

    pub fn with_middle_vertical_nodes(nodes : Vec<ProofTreeNode>) -> ProofSubtree
    {
        if nodes.is_empty() { return ProofSubtree::empty() };
        let linked_nodes = Self::link_nodes_recursively(&nodes, 0);
        return ProofSubtree { left:None, middle:Some(Box::new(linked_nodes)), right:None };
    }

    fn link_nodes_recursively(nodes : &Vec<ProofTreeNode>, index : usize) -> ProofTreeNode
    {
        if index < nodes.len()-1
        {
            let mut current_linked_node = nodes[index].clone();
            let next_linked_node = Self::link_nodes_recursively(nodes, index+1);
            current_linked_node.middle = Some(Box::new(next_linked_node));
            return current_linked_node;
        }

        return nodes[index].clone();
    }

    pub fn with_left_right_nodes(left : ProofTreeNode, right : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree { left:Some(Box::new(left)), middle:None, right:Some(Box::new(right)) };
    }
}

impl ProofTree
{
    pub fn append_subtree(&mut self, subtree : &ProofSubtree, node_id : ProofTreeNodeID)
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
