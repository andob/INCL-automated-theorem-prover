use crate::formula::{AtomicFormulaExtras, FormulaExtras, Sign};

impl AtomicFormulaExtras
{
    pub fn with_sign(&self, sign : Sign) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: self.predicate_args.clone(),
            possible_world: self.possible_world,
            sign: sign,
        }
    }
}

impl FormulaExtras
{
    pub fn with_sign(&self, sign : Sign) -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: self.possible_world,
            sign: sign,
        }
    }
}
