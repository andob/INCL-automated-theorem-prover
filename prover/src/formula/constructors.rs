use crate::formula::{AtomicFormulaExtras, FormulaExtras, PossibleWorld, PredicateArgument, PredicateArguments, Sign};

impl AtomicFormulaExtras
{
    pub fn empty() -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: PredicateArguments::empty(),
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
        }
    }

    pub fn new(args : PredicateArguments) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: args,
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
        }
    }
}

impl FormulaExtras
{
    pub fn empty() -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
        }
    }
}

impl PossibleWorld
{
    pub fn zero() -> PossibleWorld
    {
        return PossibleWorld { index:0 };
    }
}

impl PossibleWorld
{
    pub fn fork(&self) -> PossibleWorld
    {
        return PossibleWorld { index:self.index+1 };
    }
}

impl PredicateArguments
{
    pub fn empty() -> PredicateArguments
    {
        return PredicateArguments { args:vec![] };
    }

    pub fn new(args : Vec<PredicateArgument>) -> PredicateArguments
    {
        return PredicateArguments { args };
    }
}

impl PredicateArgument
{
    pub fn new(name : String) -> PredicateArgument
    {
        return PredicateArgument
        {
            type_name: name,
            instance_name: None,
        }
    }
}
