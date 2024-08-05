use anyhow::Result;
use std::fmt::{Display, Formatter};
use regex::Regex;
use strum_macros::Display;
use crate::formula::Formula;
use crate::parser::token_types::TokenTypeID;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Display)]
pub enum OperatorPrecedence
{
    Lowest, Low, Medium, High, Highest,
}

impl OperatorPrecedence
{
    pub fn incremented(&self) -> OperatorPrecedence
    {
        return match self
        {
            OperatorPrecedence::Lowest => OperatorPrecedence::Low,
            OperatorPrecedence::Low => OperatorPrecedence::Medium,
            OperatorPrecedence::Medium => OperatorPrecedence::High,
            OperatorPrecedence::High => OperatorPrecedence::Highest,
            OperatorPrecedence::Highest => OperatorPrecedence::Highest,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Display)]
pub enum TokenCategory
{
    Grouping,
    Atomic,
    UnaryOperation,
    BinaryOperation,
}

pub struct Token
{
    pub type_id : TokenTypeID,
    pub value : String,
}

impl Display for Token
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{} {}", self.type_id, self.value)
    }
}

pub struct TokenType
{
    pub id : TokenTypeID,
    pub regex : Regex,
    pub category : TokenCategory,
    pub precedence : OperatorPrecedence,
    pub to_formula : fn(String, Vec<Formula>) -> Result<Formula>,
}
