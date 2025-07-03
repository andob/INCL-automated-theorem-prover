pub mod to_string;
mod constructors;
mod collections;
pub mod converters;
pub mod notations;
mod operators;
mod random;

use smol_str::SmolStr;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Formula
{
    Atomic(SmolStr, AtomicFormulaExtras),
    Non(Box<Formula>, FormulaExtras),
    And(Box<Formula>, Box<Formula>, FormulaExtras),
    Or(Box<Formula>, Box<Formula>, FormulaExtras),
    Imply(Box<Formula>, Box<Formula>, FormulaExtras),
    BiImply(Box<Formula>, Box<Formula>, FormulaExtras),
    StrictImply(Box<Formula>, Box<Formula>, FormulaExtras),
    Conditional(Box<Formula>, Box<Formula>, FormulaExtras),
    Exists(PredicateArgument, Box<Formula>, FormulaExtras),
    ForAll(PredicateArgument, Box<Formula>, FormulaExtras),
    Equals(PredicateArgument, PredicateArgument, FormulaExtras),
    DefinitelyExists(PredicateArgument, FormulaExtras),
    Possible(Box<Formula>, FormulaExtras),
    Necessary(Box<Formula>, FormulaExtras),
    InPast(Box<Formula>, FormulaExtras),
    InFuture(Box<Formula>, FormulaExtras),
    LessThan(FuzzyTags, FuzzyTags, FormulaExtras),
    GreaterOrEqualThan(FuzzyTags, FuzzyTags, FormulaExtras),
    Comment(SmolStr),
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct AtomicFormulaExtras
{
    pub predicate_args : PredicateArguments,
    pub possible_world : PossibleWorld,
    pub sign : Sign,
    pub fuzzy_tags : FuzzyTags,
    pub is_hidden : bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FormulaExtras
{
    pub possible_world : PossibleWorld,
    pub sign : Sign,
    pub fuzzy_tags : FuzzyTags,
    pub is_hidden : bool,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct PossibleWorld
{
    pub index : u8
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum Sign
{
    Plus, Minus
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PredicateArguments
{
    args : Vec<PredicateArgument>
}

#[derive(Eq, Hash, Ord, PartialOrd, Clone)]
pub struct PredicateArgument
{
    pub variable_name : SmolStr, //variable name eg: x,y,z
    pub object_name : SmolStr, //object name eg: a,b,c
    pub is_rigid_designator : bool,
}

impl PredicateArgument
{
    pub fn new_variable(name : SmolStr) -> PredicateArgument
    {
        let is_rigid_designator = !(name == "α" || name == "β" || name == "γ");
        return PredicateArgument { variable_name:name.clone(), object_name:name, is_rigid_designator };
    }

    pub fn new_instantiated_object(object_name : SmolStr, variable_name : SmolStr) -> PredicateArgument
    {
        return PredicateArgument { variable_name, object_name, is_rigid_designator:true };
    }

    pub fn is_instantiated(&self) -> bool
    {
        return self.object_name != self.variable_name;
    }
}

impl PartialEq<Self> for PredicateArgument
{
    fn eq(&self, other : &Self) -> bool
    {
        return self.object_name == other.object_name;
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FuzzyTags
{
    tags : Vec<FuzzyTag>
}

#[derive(Eq, Hash, Ord, PartialOrd, Clone)]
pub struct FuzzyTag
{
    pub object_name : SmolStr, //eg: α,β,γ
    pub sign : Sign,
}

impl FuzzyTag
{
    pub fn get_variable_range(&self) -> (f64, f64)
    {
        return if self == &FuzzyTag::zero() { (0.0, 0.0) }
            else if self == &FuzzyTag::one() { (1.0, 1.0) }
            else { (0.0, 1.0) };
    }

    pub fn abs(&self) -> FuzzyTag
    {
        let mut absolute_tag = self.clone();
        absolute_tag.sign = Sign::Plus;
        return absolute_tag;
    }
}

impl PartialEq<Self> for FuzzyTag
{
    fn eq(&self, other : &Self) -> bool
    {
        return self.object_name == other.object_name;
    }
}
