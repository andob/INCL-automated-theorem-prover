use regex::Regex;
use anyhow::{Context, Result};
use box_macro::bx;
use strum_macros::{Display, EnumIter};
use substring::Substring;
use crate::codeloc;
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PredicateArgument, PredicateArguments};
use crate::parser::models::{OperatorPrecedence, TokenCategory, TokenType};

pub const QUANTIFIER_ANY_KEYWORD : &str = "any";

#[derive(Eq, PartialEq, Hash, Clone, Copy, EnumIter, Display)]
pub enum TokenTypeID
{
    Exists, ForAll,
    AtomicWithoutArgs, AtomicWithArgs,
    Non, And, Or, Imply, BiImply,
    Possible, Necessary, InPast, InFuture,
    StrictImply, Conditional,
    OpenParenthesis, ClosedParenthesis,
}

impl TokenType
{
    pub fn get_types() -> Result<Vec<TokenType>>
    {
        return Ok(vec!
        [
            TokenType
            {
                //matches existential quantifier: ∃x
                id: TokenTypeID::Exists,
                regex: Regex::new(r"∃[A-Za-z_]+").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |name, args|
                {
                    let formula_extras = FormulaExtras::empty();
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    let mut predicate_arg = predicate_args[0].clone();

                    if let Formula::Atomic(atomic_name, _) = &args[0] && *atomic_name == QUANTIFIER_ANY_KEYWORD
                    {
                        //this is a free variable. convert from "∃x(any)" to "∃x:any(any)"
                        predicate_arg.instance_name = Some(predicate_arg.type_name.clone());
                        predicate_arg.type_name = QUANTIFIER_ANY_KEYWORD.to_string();
                        predicate_arg.is_free = true;
                    }

                    return Formula::Exists(predicate_arg, bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches for all quantifier: ∀x
                id: TokenTypeID::ForAll,
                regex: Regex::new(r"∀[A-Za-z_]+").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |name, args|
                {
                    let formula_extras = FormulaExtras::empty();
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    return Formula::ForAll(predicate_args[0].clone(), bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches atomic formulas with args: P(x,y), ...
                id: TokenTypeID::AtomicWithArgs,
                regex: Regex::new(r"[A-Za-z_]+\[[A-Za-z_,]+\]").context(codeloc!())?,
                category: TokenCategory::Atomic,
                precedence: OperatorPrecedence::Lowest,
                to_formula: |name, args|
                {
                    let atomic_name = name.substring(0, name.find('[').unwrap_or(1)).to_string();
                    let predicate_args = Self::parse_predicate_arguments(&name);
                    let formula_extras = AtomicFormulaExtras::new(predicate_args);
                    return Formula::Atomic(atomic_name, formula_extras);
                },
            },

            TokenType
            {
                //matches atomic formulas: P, Q, ...
                id: TokenTypeID::AtomicWithoutArgs,
                regex: Regex::new(r"[A-Za-z_]+").context(codeloc!())?,
                category: TokenCategory::Atomic,
                precedence: OperatorPrecedence::Lowest,
                to_formula: |name,_|
                {
                    let formula_extras = AtomicFormulaExtras::empty();
                    return Formula::Atomic(name, formula_extras);
                },
            },

            TokenType
            {
                //matches not: ~P, ~Q, ...
                id: TokenTypeID::Non,
                regex: Regex::new(r"(~)|(¬)|(!)").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Non(bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches possible: ◇P, ◇Q, ...
                id: TokenTypeID::Possible,
                regex: Regex::new(r"(◇)").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Possible(bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches necessary: □P, □Q, ...
                id: TokenTypeID::Necessary,
                regex: Regex::new(r"(□)").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Necessary(bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches the past: ᵖ
                id: TokenTypeID::InPast,
                regex: Regex::new(r"(ᵖ)").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::InPast(bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches the future: ᶠ
                id: TokenTypeID::InFuture,
                regex: Regex::new(r"(ᶠ)").context(codeloc!())?,
                category: TokenCategory::UnaryOperation,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::InFuture(bx!(args[0].clone()), formula_extras);
                },
            },

            TokenType
            {
                //matches and: P & Q
                id: TokenTypeID::And,
                regex: Regex::new(r"(&)|(∧)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::High,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::And(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches or: P | Q
                id: TokenTypeID::Or,
                regex: Regex::new(r"(\|)|(∨)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::High,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Or(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches imply: P → Q
                id: TokenTypeID::Imply,
                regex: Regex::new(r"(→)|(⇒)|(⊃)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Medium,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Imply(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches equivalence: P ≡ Q
                id: TokenTypeID::BiImply,
                regex: Regex::new(r"(↔)|(⇔)|(≡)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Low,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::BiImply(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches strict implication: P ⥽ Q
                id: TokenTypeID::StrictImply,
                regex: Regex::new(r"(⥽)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Low,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::StrictImply(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches conditional: P ᐅ Q
                id: TokenTypeID::Conditional,
                regex: Regex::new(r"(ᐅ)").context(codeloc!())?,
                category: TokenCategory::BinaryOperation,
                precedence: OperatorPrecedence::Low,
                to_formula: |_,args|
                {
                    let formula_extras = FormulaExtras::empty();
                    return Formula::Conditional(bx!(args[0].clone()), bx!(args[1].clone()), formula_extras);
                }
            },

            TokenType
            {
                //matches open parenthesis
                id: TokenTypeID::OpenParenthesis,
                regex: Regex::new(r"\(").context(codeloc!())?,
                category: TokenCategory::Grouping,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    panic!("Cannot convert ( to formula!");
                }
            },

            TokenType
            {
                //matches closed parenthesis
                id: TokenTypeID::ClosedParenthesis,
                regex: Regex::new(r"\)").context(codeloc!())?,
                category: TokenCategory::Grouping,
                precedence: OperatorPrecedence::Highest,
                to_formula: |_,args|
                {
                    panic!("Cannot convert ) to formula!");
                }
            },
        ]);
    }

    fn parse_predicate_arguments(input : &String) -> PredicateArguments
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

        return PredicateArguments::new(input.split(",")
            .map(|token| PredicateArgument::new(token.trim().to_string()))
            .collect());
    }
}
