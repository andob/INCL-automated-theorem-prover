use std::ops::Index;
use std::slice::Iter;
use std::vec::IntoIter;
use crate::formula::{FuzzyTag, FuzzyTags, PredicateArgument, PredicateArguments};

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

impl FuzzyTags
{
    pub fn is_empty(&self) -> bool
    {
        return self.tags.is_empty();
    }

    pub fn iter(&self) -> Iter<'_, FuzzyTag>
    {
        return self.tags.iter();
    }

    pub fn into_iter(self) -> IntoIter<FuzzyTag>
    {
        return self.tags.into_iter();
    }

    pub fn len(&self) -> usize
    {
        return self.tags.len();
    }

    pub fn plus(&self, new_tag : FuzzyTag) -> FuzzyTags
    {
        let mut tags = self.tags.clone();
        tags.push(new_tag);
        return FuzzyTags::new(tags);
    }
}

impl Index<usize> for FuzzyTags
{
    type Output = FuzzyTag;
    fn index(&self, index : usize) -> &Self::Output
    {
        return &self.tags[index];
    }
}
