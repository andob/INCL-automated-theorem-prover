pub mod to_string;
mod constructors;
mod collections;
pub mod converters;
pub mod notations;
mod operators;

use std::fmt::Display;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Formula
{
    Atomic(String, AtomicFormulaExtras),
    Non(Box<Formula>, FormulaExtras),
    And(Box<Formula>, Box<Formula>, FormulaExtras),
    Or(Box<Formula>, Box<Formula>, FormulaExtras),
    Imply(Box<Formula>, Box<Formula>, FormulaExtras),
    BiImply(Box<Formula>, Box<Formula>, FormulaExtras),
    StrictImply(Box<Formula>, Box<Formula>, FormulaExtras),
    Conditional(Box<Formula>, Box<Formula>, FormulaExtras),
    Exists(PredicateArgument, Box<Formula>, FormulaExtras),
    ForAll(PredicateArgument, Box<Formula>, FormulaExtras),
    Possible(Box<Formula>, FormulaExtras),
    Necessary(Box<Formula>, FormulaExtras),
    InPast(Box<Formula>, FormulaExtras),
    InFuture(Box<Formula>, FormulaExtras),
    Comment(String),
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct AtomicFormulaExtras
{
    pub predicate_args : PredicateArguments,
    pub possible_world : PossibleWorld,
    pub is_hidden : bool,
    pub sign : Sign,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FormulaExtras
{
    pub possible_world : PossibleWorld,
    pub is_hidden : bool,
    pub sign : Sign,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct PossibleWorld
{
    pub index : i8
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Sign
{
    Plus, Minus
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PredicateArguments
{
    args : Vec<PredicateArgument>
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PredicateArgument
{
    pub type_name : String, //variables eg: x,y,z
    pub instance_name : Option<String>, //variable instances eg: a,b,c
    pub is_free : bool, //is a free variable?
}
