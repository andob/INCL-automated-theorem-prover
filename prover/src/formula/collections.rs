use std::ops::Index;
use std::slice::Iter;
use std::vec::IntoIter;
use crate::formula::{PredicateArgument, PredicateArguments};

impl PredicateArguments
{
    pub fn is_empty(&self) -> bool
    {
        return self.args.is_empty();
    }

    pub fn iter(&self) -> Iter<'_, PredicateArgument>
    {
        return self.args.iter();
    }

    pub fn into_iter(self) -> IntoIter<PredicateArgument>
    {
        return self.args.into_iter();
    }

    pub fn len(&self) -> usize
    {
        return self.args.len();
    }
}

impl Index<usize> for PredicateArguments
{
    type Output = PredicateArgument;
    fn index(&self, index : usize) -> &Self::Output
    {
        return &self.args[index];
    }
}
