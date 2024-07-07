use crate::formula::{AtomicFormulaExtras, FormulaExtras};

impl AtomicFormulaExtras
{
    pub fn with_is_hidden(&self, is_hidden : bool) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: self.predicate_args.clone(),
            possible_world: self.possible_world,
            is_hidden: is_hidden,
            sign: self.sign,
        }
    }
}

impl FormulaExtras
{
    pub fn with_is_hidden(&self, is_hidden : bool) -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: self.possible_world,
            is_hidden: is_hidden,
            sign: self.sign,
        }
    }
}
