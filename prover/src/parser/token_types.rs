use regex::Regex;
use strum_macros::Display;
use substring::Substring;
use crate::{Formula, PredicateArgument};
use crate::parser::models::{OperatorPrecedence, TokenCategory, TokenType};

#[derive(Copy, Clone, Eq, PartialEq, Display)]
pub enum TokenTypeID
{
    Exists, ForAll,
    AtomicWithoutArgs, AtomicWithArgs,
    Non, And, Or, Imply, BiImply,
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
                //matches existential quantifier: ∃x
                id: TokenTypeID::Exists,
                regex: Regex::new(r"∃[A-Za-z_]+").unwrap(),
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |name, args|
                {
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    return Formula::Exists(predicate_args[0].clone(), args[0].to_box());
                },
            },

            TokenType
            {
                //matches for all quantifier: ∀x
                id: TokenTypeID::ForAll,
                regex: Regex::new(r"∀[A-Za-z_]+").unwrap(),
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |name, args|
                {
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    return Formula::ForAll(predicate_args[0].clone(), args[0].to_box());
                },
            },

            TokenType
            {
                //matches atomic formulas with args: P(x,y), ...
                id: TokenTypeID::AtomicWithArgs,
                regex: Regex::new(r"[A-Za-z_]+\[[A-Za-z_,]+\]").unwrap(),
                category: TokenCategory::Atomic,
                precedence: OperatorPrecedence::Lowest,
                to_formula: |name, args|
                {
                    let atomic_name = name.substring(0, name.find('[').unwrap()).to_string();
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    return Formula::Atomic(atomic_name, predicate_args);
                },
            },

            TokenType
            {
                //matches atomic formulas: P, Q, ...
                id: TokenTypeID::AtomicWithoutArgs,
                regex: Regex::new(r"[A-Za-z_]+").unwrap(),
                category: TokenCategory::Atomic,
                precedence: OperatorPrecedence::Lowest,
                to_formula: |name,_| { Formula::Atomic(name, vec![]) },
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

    fn parse_predicate_arguments(input : &String) -> Vec<PredicateArgument>
    {
        if let Some(index_of_open_bracket) = input.find('[')
        {
            if let Some(index_of_closed_bracket) = input.find(']')
            {
                let new_input = input.substring(index_of_open_bracket+1, index_of_closed_bracket).to_string();
                return Self::parse_predicate_arguments(&new_input);
            }
        }

        if let Some(index_of_exists) = input.find('∃')
        {
            let new_input = input.substring(index_of_exists+1, input.len()-1).to_string();
            return Self::parse_predicate_arguments(&new_input);
        }

        if let Some(index_of_for_all) = input.find('∀')
        {
            let new_input = input.substring(index_of_for_all+1, input.len()-1).to_string();
            return Self::parse_predicate_arguments(&new_input);
        }

        return input.split(",").map(|token| token.trim().to_string())
                .map(|name| PredicateArgument::new(name)).collect();
    }
}
