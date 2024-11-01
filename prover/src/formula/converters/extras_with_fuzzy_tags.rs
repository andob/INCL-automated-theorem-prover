use crate::formula::{AtomicFormulaExtras, FormulaExtras, FuzzyTags};

impl AtomicFormulaExtras
{
    pub fn with_fuzzy_tags(&self, tags : FuzzyTags) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: self.predicate_args.clone(),
            possible_world: self.possible_world.clone(),
            sign: self.sign,
            fuzzy_tags: tags,
            is_hidden: self.is_hidden,
        }
    }
}

impl FormulaExtras
{
    pub fn with_fuzzy_tags(&self, tags : FuzzyTags) -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: self.possible_world.clone(),
            sign: self.sign,
            fuzzy_tags: tags,
            is_hidden: self.is_hidden,
        }
    }
}
