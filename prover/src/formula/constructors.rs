use substring::Substring;
use crate::formula::{AtomicFormulaExtras, FormulaExtras, FuzzyTag, FuzzyTags, PossibleWorld, PredicateArgument, PredicateArguments, Sign};
use crate::formula::Sign::Plus;

impl AtomicFormulaExtras
{
    pub fn empty() -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: PredicateArguments::empty(),
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
            fuzzy_tags: FuzzyTags::empty(),
            is_hidden: false,
        }
    }

    pub fn new(args : PredicateArguments) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: args,
            possible_world: PossibleWorld::zero(),
            sign: Sign::Plus,
            fuzzy_tags: FuzzyTags::empty(),
            is_hidden: false,
        }
    }

    pub fn from(extras : &FormulaExtras) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: PredicateArguments::empty(),
            possible_world: extras.possible_world,
            sign: extras.sign,
            fuzzy_tags: extras.fuzzy_tags.clone(),
            is_hidden: extras.is_hidden,
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
            fuzzy_tags: FuzzyTags::empty(),
            is_hidden: false,
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
        if name.contains(':')
        {
            //this is an instantiated predicate argument
            let index_of_colon = name.find(':').unwrap();
            let object_name = name.substring(0, index_of_colon).to_string();
            let variable_name = name.substring(index_of_colon+1, name.len()).to_string();
            return PredicateArgument { variable_name, object_name };
        }

        //this is an uninstantiated predicate argument
        return PredicateArgument { variable_name:name.to_string(), object_name:name }
    }
}

impl FuzzyTags
{
    pub fn empty() -> FuzzyTags
    {
        return FuzzyTags { tags:vec![] };
    }

    pub fn new(tags : Vec<FuzzyTag>) -> FuzzyTags
    {
        return FuzzyTags { tags };
    }
}

impl FuzzyTag
{
    pub fn new(name : String) -> FuzzyTag
    {
        return FuzzyTag { object_name:name, sign:Plus };
    }

    pub fn zero() -> FuzzyTag
    {
        let object_name = String::from('0');
        return FuzzyTag { object_name, sign:Plus };
    }

    pub fn one() -> FuzzyTag
    {
        let object_name = String::from('1');
        return FuzzyTag { object_name, sign:Plus };
    }
}
