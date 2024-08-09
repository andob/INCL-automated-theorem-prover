use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::{ProofTreeNodeFactory, ProofTreeNodeID};
use crate::tree::ProofTree;

#[derive(Clone)]
pub struct ProofSubtree
{
    pub left : Option<Box<ProofTreeNode>>,
    pub middle : Option<Box<ProofTreeNode>>,
    pub right : Option<Box<ProofTreeNode>>,
    cloned_subtrees_with_new_ids : Vec<Box<ProofSubtree>>,
}

impl ProofSubtree
{
    fn new(left : Option<Box<ProofTreeNode>>, middle : Option<Box<ProofTreeNode>>, right : Option<Box<ProofTreeNode>>) -> ProofSubtree
    {
        return ProofSubtree { left, middle, right, cloned_subtrees_with_new_ids:vec![] };
    }

    pub fn empty() -> ProofSubtree
    {
        return ProofSubtree::new(None, None, None);
    }

    pub fn with_middle_node(node : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree::new(None, Some(Box::new(node)), None);
    }

    pub fn with_middle_vertical_nodes(nodes : Vec<ProofTreeNode>) -> ProofSubtree
    {
        if nodes.is_empty() { return ProofSubtree::empty() };
        let linked_nodes = Self::link_nodes_recursively(&nodes, 0);
        return ProofSubtree::new(None, Some(Box::new(linked_nodes)), None);
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
        return ProofSubtree::new(Some(Box::new(left)), None, Some(Box::new(right)));
    }

    pub fn with_left_middle_right_nodes(left : ProofTreeNode, middle : ProofTreeNode, right : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree::new(Some(Box::new(left)), Some(Box::new(middle)), Some(Box::new(right)));
    }

    fn attach_new_ids(&mut self, node_factory : &mut ProofTreeNodeFactory)
    {
        if let Some(left) = &mut self.left { left.attach_new_ids(node_factory); }
        if let Some(middle) = &mut self.middle { middle.attach_new_ids(node_factory); }
        if let Some(right) = &mut self.right { right.attach_new_ids(node_factory); }
    }

    pub fn append(&mut self, another_subtree : ProofSubtree)
    {
        let root_nodes_refs = [&mut self.left, &mut self.middle, &mut self.right];
        for root_node_ref in root_nodes_refs
        {
            if let Some(ref mut root_node) = root_node_ref
            {
                let leaf_node_ids = root_node.get_all_paths().iter()
                    .map(|path| path.get_leaf_node_id())
                    .collect::<Vec<ProofTreeNodeID>>();

                for leaf_node_id in leaf_node_ids
                {
                    root_node.append_subtree_recursive(&another_subtree, leaf_node_id);
                }
            }
        }
    }
}

impl ProofTree
{
    pub fn append_subtree(&mut self, subtree : &mut ProofSubtree, node_id : ProofTreeNodeID)
    {
        let mut should_clone_subtree_with_new_ids = false;

        let paths = self.get_all_paths();
        for path in &paths
        {
            let leaf = path.nodes.last().unwrap();
            if !leaf.is_contradictory && (leaf.id == node_id || path.nodes.iter().any(|node| node.id == node_id))
            {
                if !should_clone_subtree_with_new_ids
                {
                    self.root_node.append_subtree_recursive(&subtree, leaf.id);
                    should_clone_subtree_with_new_ids = true;
                }
                else
                {
                    let mut subtree_with_new_ids = subtree.clone();
                    subtree_with_new_ids.attach_new_ids(&mut self.node_factory);
                    self.root_node.append_subtree_recursive(&subtree_with_new_ids, leaf.id);
                    subtree.cloned_subtrees_with_new_ids.push(Box::new(subtree_with_new_ids));
                }
            }
        }
    }
}

impl ProofTreeNode
{
    fn append_subtree_recursive(&mut self, subtree : &ProofSubtree, node_id : ProofTreeNodeID)
    {
        if self.id == node_id
        {
            self.left = subtree.left.clone();
            self.middle = subtree.middle.clone();
            self.right = subtree.right.clone();
        }
        else
        {
            if let Some(left) = &mut self.left { left.append_subtree_recursive(subtree, node_id); }
            if let Some(middle) = &mut self.middle { middle.append_subtree_recursive(subtree, node_id); }
            if let Some(right) = &mut self.right { right.append_subtree_recursive(subtree, node_id); }
        }
    }

    fn attach_new_ids(&mut self, node_factory: &mut ProofTreeNodeFactory)
    {
        self.id = node_factory.new_node_id();
        if let Some(left) = &mut self.left { left.attach_new_ids(node_factory); }
        if let Some(middle) = &mut self.middle { middle.attach_new_ids(node_factory); }
        if let Some(right) = &mut self.right { right.attach_new_ids(node_factory); }
    }
}

impl DecompositionPriorityQueue
{
    pub fn push_subtree(&mut self, subtree : Box<ProofSubtree>)
    {
        if let Some(left) = subtree.left { self.push_tree_node(left); }
        if let Some(middle) = subtree.middle { self.push_tree_node(middle); }
        if let Some(right) = subtree.right { self.push_tree_node(right); }

        for alternative_subtree_data in subtree.cloned_subtrees_with_new_ids
        {
            self.push_subtree(alternative_subtree_data);
        }
    }
}
