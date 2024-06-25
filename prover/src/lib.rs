mod parser;

use anyhow::{Result, Context};
use std::fmt::{Display, format, Formatter};
use substring::Substring;
use crate::parser::algorithm::LogicalExpressionParser;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

#[derive(Clone)]
enum Formula
{
    Atomic(String),
    Non(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Imply(Box<Formula>, Box<Formula>),
    BiImply(Box<Formula>, Box<Formula>),
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
        let subformula_to_string = |f : &Box<Formula>| f.to_string_impl(index+1);

        let string = match self
        {
            Formula::Atomic(p) => { format!("{}", p) }
            Formula::Non(x) => { format!("¬{}", subformula_to_string(x)) }
            Formula::And(x, y) => { format!("({} ∧ {})", subformula_to_string(x), subformula_to_string(y)) }
            Formula::Or(x, y) => { format!("({} ∨ {})", subformula_to_string(x), subformula_to_string(y)) }
            Formula::Imply(x, y) => { format!("({} ⊃ {})", subformula_to_string(x), subformula_to_string(y)) }
            Formula::BiImply(x, y) => { format!("({} ≡ {})", subformula_to_string(x), subformula_to_string(y)) }
        };

        if index==0
        {
            return string.substring(1, string.len()-2).to_string();
        }

        return string;
    }
}

impl Display for Formula
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.to_string_impl(0));
    }
}

pub fn test(input : String) -> Result<()>
{
    let formula = LogicalExpressionParser::parse(input)?;
    println!("{}", formula);
    return Ok(());
}
