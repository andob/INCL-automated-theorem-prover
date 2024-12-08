use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::{ProofTreeNodeFactory, ProofTreeNodeID};
use crate::tree::ProofTree;

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

    pub fn is_empty(&self) -> bool
    {
        return self.left.is_none() && self.middle.is_none() && self.right.is_none();
    }

    pub fn with_middle_node(node : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree::new(None, Some(Box::new(node)), None);
    }

    pub fn with_middle_vertical_nodes(nodes : Vec<ProofTreeNode>) -> ProofSubtree
    {
        if nodes.is_empty() { return ProofSubtree::empty() };
        let middle = Self::link_middle_nodes_recursively(&nodes, 0);
        return ProofSubtree::new(None, Some(Box::new(middle)), None);
    }

    fn link_middle_nodes_recursively(nodes : &Vec<ProofTreeNode>, index : usize) -> ProofTreeNode
    {
        if index < nodes.len()-1
        {
            let mut current_linked_node = nodes[index].clone();
            let next_linked_node = Self::link_middle_nodes_recursively(nodes, index+1);
            current_linked_node.middle = Some(Box::new(next_linked_node));
            return current_linked_node;
        }

        return nodes[index].clone();
    }

    pub fn with_zipped_left_right_vertical_nodes(nodes : Vec<(ProofTreeNode, ProofTreeNode)>, node_factory : &mut ProofTreeNodeFactory) -> ProofSubtree
    {
        if nodes.is_empty() { return ProofSubtree::empty() };

        let (mut root_left, mut root_right) = nodes.first().cloned().unwrap();
        let (mut id_on_left, mut id_on_right) = (root_left.id, root_right.id);

        for index in 1..nodes.len()
        {
            let (left, right) = nodes[index].clone();
            let (next_id_on_left, next_id_on_right) = (left.id, right.id);
            let subtree = ProofSubtree::new(Some(Box::new(left)), None, Some(Box::new(right)));
            root_left.append_subtree_on_leaf(&subtree, id_on_left);
            root_right.append_subtree_on_leaf(&subtree, id_on_right);
            id_on_left = next_id_on_left;
            id_on_right = next_id_on_right;
        }

        root_left.attach_new_ids(node_factory);
        root_right.attach_new_ids(node_factory);

        return ProofSubtree::new(Some(Box::new(root_left)), None, Some(Box::new(root_right)));
    }

    pub fn with_left_right_nodes(left : ProofTreeNode, right : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree::new(Some(Box::new(left)), None, Some(Box::new(right)));
    }

    pub fn with_left_middle_right_nodes(left : ProofTreeNode, middle : ProofTreeNode, right : ProofTreeNode) -> ProofSubtree
    {
        return ProofSubtree::new(Some(Box::new(left)), Some(Box::new(middle)), Some(Box::new(right)));
    }

    pub fn with_hidden_nodes(mut self) -> ProofSubtree
    {
        if let Some(left) = &mut self.left { left.hide_all_nodes(); }
        if let Some(middle) = &mut self.middle { middle.hide_all_nodes(); }
        if let Some(right) = &mut self.right { right.hide_all_nodes(); }
        return self;
    }

    fn attach_new_ids(&mut self, node_factory : &mut ProofTreeNodeFactory)
    {
        if let Some(left) = &mut self.left { left.attach_new_ids(node_factory); }
        if let Some(middle) = &mut self.middle { middle.attach_new_ids(node_factory); }
        if let Some(right) = &mut self.right { right.attach_new_ids(node_factory); }
    }

    pub fn append(&mut self, another_subtree : &ProofSubtree)
    {
        if !another_subtree.is_empty()
        {
            if let Some(ref mut left) = &mut self.left
                { left.append_subtree(&another_subtree); }
            else { self.left = another_subtree.left.clone(); }

            if let Some(ref mut middle) = &mut self.middle
                { middle.append_subtree(&another_subtree); }
            else { self.middle = another_subtree.middle.clone(); }

            if let Some(ref mut right) = &mut self.right
                { right.append_subtree(&another_subtree); }
            else { self.right = another_subtree.right.clone(); }
        }
    }
}

impl ProofTree
{
    pub fn append_subtree(&mut self, another_subtree : &mut ProofSubtree, target_node_id : ProofTreeNodeID)
    {
        if another_subtree.is_empty() { return }

        let mut should_clone_subtree_with_new_ids = false;

        let paths = self.get_all_paths();
        for path in &paths
        {
            let leaf = path.nodes.last().unwrap();
            if !leaf.is_contradictory && (leaf.id == target_node_id ||
                path.nodes.iter().any(|node| node.id == target_node_id))
            {
                if !should_clone_subtree_with_new_ids
                {
                    self.root_node.append_subtree_on_leaf(&another_subtree, leaf.id);
                    should_clone_subtree_with_new_ids = true;
                }
                else
                {
                    let mut subtree_with_new_ids = another_subtree.clone();
                    subtree_with_new_ids.attach_new_ids(&mut self.node_factory);
                    self.root_node.append_subtree_on_leaf(&subtree_with_new_ids, leaf.id);
                    another_subtree.cloned_subtrees_with_new_ids.push(Box::new(subtree_with_new_ids));
                }
            }
        }
    }
}

impl ProofTreeNode
{
    fn append_subtree(&mut self, another_subtree: &ProofSubtree)
    {
        let leaf_node_ids = self.get_all_paths().iter()
            .map(|path| path.get_leaf_node_id())
            .collect::<Vec<ProofTreeNodeID>>();

        for leaf_node_id in leaf_node_ids
        {
            self.append_subtree_on_leaf(another_subtree, leaf_node_id);
        }
    }

    fn append_subtree_on_leaf(&mut self, subtree : &ProofSubtree, node_id : ProofTreeNodeID)
    {
        if self.id == node_id
        {
            self.left = subtree.left.clone();
            self.middle = subtree.middle.clone();
            self.right = subtree.right.clone();
        }
        else
        {
            if let Some(left) = &mut self.left { left.append_subtree_on_leaf(subtree, node_id); }
            if let Some(middle) = &mut self.middle { middle.append_subtree_on_leaf(subtree, node_id); }
            if let Some(right) = &mut self.right { right.append_subtree_on_leaf(subtree, node_id); }
        }
    }

    fn hide_all_nodes(&mut self)
    {
        self.formula = self.formula.with_is_hidden(true);
        if let Some(left) = &mut self.left { left.hide_all_nodes(); }
        if let Some(middle) = &mut self.middle { middle.hide_all_nodes(); }
        if let Some(right) = &mut self.right { right.hide_all_nodes(); }
    }

    fn attach_new_ids(&mut self, node_factory : &mut ProofTreeNodeFactory)
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
        if subtree.is_empty() { return }

        if let Some(left) = subtree.left { self.push_tree_node(left); }
        if let Some(middle) = subtree.middle { self.push_tree_node(middle); }
        if let Some(right) = subtree.right { self.push_tree_node(right); }

        for alternative_subtree_data in subtree.cloned_subtrees_with_new_ids
        {
            self.push_subtree(alternative_subtree_data);
        }
    }
}

impl Clone for ProofSubtree
{
    fn clone(&self) -> Self
    {
        return ProofSubtree
        {
            left: self.left.clone(), middle: self.middle.clone(), right: self.right.clone(),
            cloned_subtrees_with_new_ids: vec![],
        }
    }
}

impl Default for ProofSubtree
{
    fn default() -> Self
    {
        return ProofSubtree::empty();
    }
}
