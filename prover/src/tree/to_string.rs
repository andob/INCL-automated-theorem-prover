use std::fmt::{Display, Formatter};
use crate::formula::to_string::FormulaFormatOptions;
use crate::tree::node::ProofTreeNode;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

impl Display for ProofTree
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let options = FormulaFormatOptions::recommended_for(&self.problem.logic);
        return write!(f, "{}", self.to_string_with_options(&options));
    }
}

impl ProofTree
{
    pub fn to_string_with_options(&self, options : &FormulaFormatOptions) -> String
    {
        let mut output_string = String::new();

        if self.has_timeout { output_string.push_str("TIMEOUT!\n"); }
        else if self.is_proof_correct { output_string.push_str("PROVED!\n"); }
        else { output_string.push_str("NOT PROVED!\n"); }

        self.root_node.print_as_subtree_to_string(options, &mut output_string, 0);

        return output_string;
    }
}

impl Display for ProofSubtree
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let options = FormulaFormatOptions::default();
        return write!(f, "{}", self.to_string_with_options(&options));
    }
}

impl ProofSubtree
{
    pub fn to_string_with_options(&self, options : &FormulaFormatOptions) -> String
    {
        if self.is_empty() { return String::from("Empty subtree") }

        let mut output_string = String::from("Subtree\n");

        if let Some(left) = &self.left { left.print_as_subtree_to_string(&options, &mut output_string, 1) }
        if let Some(middle) = &self.middle { middle.print_as_subtree_to_string(&options, &mut output_string, 1) }
        if let Some(right) = &self.right { right.print_as_subtree_to_string(&options, &mut output_string, 1) }

        return output_string;
    }
}

impl ProofTreeNode
{
    fn print_as_subtree_to_string(&self, options : &FormulaFormatOptions, out_string : &mut String, indent : usize)
    {
        if indent>0
        {
            //append tree glyphs
            out_string.push('├');
            for _ in 0..indent { out_string.push_str("──"); }
            out_string.push(' ');
        }

        //append node ID
        out_string.push('<');
        out_string.push_str(self.id.to_string().as_str());
        out_string.push('>');
        out_string.push(' ');

        //append formula
        let formula_as_string = self.formula.to_string_with_options(options);
        out_string.push_str(formula_as_string.replace("\n", " ").as_str());

        if self.is_contradictory
        {
            //append contradiction sign
            out_string.push(' ');
            out_string.push('X');
        }

        out_string.push('\n');

        if let Some(left) = &self.left
        {
            left.print_as_subtree_to_string(options, out_string, indent+1);
        }

        if let Some(middle) = &self.middle
        {
            middle.print_as_subtree_to_string(options, out_string, indent+1);
        }

        if let Some(right) = &self.right
        {
            right.print_as_subtree_to_string(options, out_string, indent+1);
        }
    }
}
