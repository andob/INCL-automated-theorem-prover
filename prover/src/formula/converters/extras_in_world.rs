use crate::formula::{AtomicFormulaExtras, FormulaExtras, PossibleWorld};

impl AtomicFormulaExtras
{
    pub fn in_world(&self, possible_world : PossibleWorld) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: self.predicate_args.clone(),
            possible_world: possible_world,
            is_hidden: self.is_hidden,
            sign: self.sign,
        }
    }
}

impl FormulaExtras
{
    pub fn in_world(&self, possible_world : PossibleWorld) -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: possible_world,
            is_hidden: self.is_hidden,
            sign: self.sign,
        }
    }
}
