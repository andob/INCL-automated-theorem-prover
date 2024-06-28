use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Formula
{
    Atomic(String, Vec<PredicateArgument>),
    Non(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Imply(Box<Formula>, Box<Formula>),
    BiImply(Box<Formula>, Box<Formula>),
    Exists(PredicateArgument, Box<Formula>),
    ForAll(PredicateArgument, Box<Formula>),
    Possible(Box<Formula>),
    Necessary(Box<Formula>),
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PredicateArgument { name : String }
impl PredicateArgument
{
    pub fn new(name : String) -> PredicateArgument { PredicateArgument { name } }
}

impl Display for PredicateArgument
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.name);
    }
}

impl Formula
{
    pub fn to_box(&self) -> Box<Formula>
    {
        return Box::new(self.clone());
    }
}

impl Formula
{
    fn to_string_impl(&self, index : usize) -> String
    {
        let format_predicate_args = |args : &Vec<PredicateArgument>|
        args.iter().map(|arg|arg.name.clone()).intersperse(String::from(",")).collect::<String>();

        let format_atomic = |p : &String, args : &Vec<PredicateArgument>|
        if args.is_empty() { p.clone() } else { format!("{}[{}]", p, format_predicate_args(args)) };

        let format_unary_formula = |operator : &str, x : &Formula|
        format!("{}{}", operator, x.to_string_impl(index+1));

        let format_quantifier_formula = |operator : &str, x : &PredicateArgument, p : &Box<Formula>|
        format!("{}{}({})", operator, x, p.to_string_impl(index+1));

        let format_binary_formula = |x : &Box<Formula>, operator : &str, y : &Box<Formula>|
        if index==0 { format!("{} {} {}", x.to_string_impl(index+1), operator, y.to_string_impl(index+1)) }
        else { format!("({} {} {})", x.to_string_impl(index+1), operator, y.to_string_impl(index+1)) };

        return match self
        {
            Formula::Atomic(p, args) => { format_atomic(p, args) }
            Formula::Non(p) => { format_unary_formula("¬", p) }
            Formula::And(p, q) => { format_binary_formula(p, "∧", q) }
            Formula::Or(p, q) => { format_binary_formula(p, "∨", q) }
            Formula::Imply(p, q) => { format_binary_formula(p, "⊃", q) }
            Formula::BiImply(p, q) => { format_binary_formula(p, "≡", q) }
            Formula::Exists(x, p) => { format_quantifier_formula("∃", x, p) }
            Formula::ForAll(x, p) => { format_quantifier_formula("∀", x, p) }
            Formula::Possible(p) => { format_unary_formula("◇", p) }
            Formula::Necessary(p) => { format_unary_formula("□", p) }
        };
    }
}

impl Display for Formula
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.to_string_impl(0));
    }
}
