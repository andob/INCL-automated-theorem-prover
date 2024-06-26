use std::fmt::{Display, Formatter};
use crate::tree::node::ProofTreeNode;
use crate::tree::ProofTree;

impl<'a> Display for ProofTree<'a>
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        if self.has_timeout { writeln!(f, "TIMEOUT!")?; }
        else if self.is_proof_correct { writeln!(f, "PROVED!")?; }
        else { writeln!(f, "NOT PROVED!")?; }

        let mut tree = String::new();
        self.root_node.print_as_subtree_to_string(&mut tree, 0);
        return writeln!(f, "{}", tree);
    }
}

impl ProofTreeNode
{
    fn print_as_subtree_to_string(&self, out_string : &mut String, indent : usize)
    {
        if indent>0
        {
            out_string.push('├');
            for _ in 0..indent { out_string.push_str("──"); }
            out_string.push(' ');
        }

        out_string.push_str(self.formula.to_string().as_str());

        //todo if options.should_show_possible_worlds && self.formula !is ModalRelationDescriptorFormula
        // {
        //     out_string.push_str(format!(" {}", self.formula.possible_world));
        // }

        if self.is_contradictory
        {
            out_string.push_str(" X");
        }

        out_string.push('\n');

        if let Some(left) = &self.left
        {
            left.print_as_subtree_to_string(out_string, indent+1);
        }

        if let Some(middle) = &self.middle
        {
            middle.print_as_subtree_to_string(out_string, indent+1);
        }

        if let Some(right) = &self.right
        {
            right.print_as_subtree_to_string(out_string, indent+1);
        }
    }
}
