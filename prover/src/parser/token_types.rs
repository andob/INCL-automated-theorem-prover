use regex::Regex;
use strum_macros::Display;
use crate::Formula;
use crate::parser::models::{OperatorPrecedence, TokenCategory, TokenType};

#[derive(Copy, Clone, Eq, PartialEq, Display)]
pub enum TokenTypeID
{
    Atomic, Non, And, Or, Imply, BiImply,
    OpenParenthesis, ClosedParenthesis,
}

impl TokenType
{
    pub fn get_types() -> Vec<TokenType>
    {
        return vec!
        [
            TokenType
            {
                //matches atomic formulas: P, Q, ...
                id: TokenTypeID::Atomic,
                regex: Regex::new(r"\b\w\b").unwrap(),
                category: TokenCategory::Atomic,
                precedence: OperatorPrecedence::Lowest,
                to_formula: |name,_| { Formula::Atomic(name) },
            },

            TokenType
            {
                //matches not: ~P, ~Q, ...
                id: TokenTypeID::Non,
                regex: Regex::new(r"(~)|(¬)|(!)").unwrap(),
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args| { Formula::Non(args[0].to_box()) },
            },

            TokenType
            {
                //matches and: P & Q
                id: TokenTypeID::And,
                regex: Regex::new(r"(&)|(∧)|(\^)").unwrap(),
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::High,
                to_formula: |_,args| { Formula::And(args[0].to_box(), args[1].to_box()) }
            },

            TokenType
            {
                //matches or: P | Q
                id: TokenTypeID::Or,
                regex: Regex::new(r"(\|)|(∨)").unwrap(),
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::High,
                to_formula: |_,args| { Formula::Or(args[0].to_box(), args[1].to_box()) }
            },

            TokenType
            {
                //matches imply: P → Q
                id: TokenTypeID::Imply,
                regex: Regex::new(r"(→)|(⇒)|(⊃)").unwrap(),
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Medium,
                to_formula: |_,args| { Formula::Imply(args[0].to_box(), args[1].to_box()) }
            },

            TokenType
            {
                //matches equivalence: P ≡ Q
                id: TokenTypeID::BiImply,
                regex: Regex::new(r"(↔)|(⇔)|(≡)").unwrap(),
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Low,
                to_formula: |_,args| { Formula::BiImply(args[0].to_box(), args[1].to_box()) }
            },

            TokenType
            {
                //matches open parenthesis
                id: TokenTypeID::OpenParenthesis,
                regex: Regex::new(r"\(").unwrap(),
                category: TokenCategory::Grouping,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args| { panic!("Cannot convert ( to formula!") }
            },

            TokenType
            {
                //matches closed parenthesis
                id: TokenTypeID::ClosedParenthesis,
                regex: Regex::new(r"\)").unwrap(),
                category: TokenCategory::Grouping,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args| { panic!("Cannot convert ) to formula!") }
            },
        ];
    }
}
