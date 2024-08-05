use crate::formula::PredicateArgument;

impl PredicateArgument
{
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
