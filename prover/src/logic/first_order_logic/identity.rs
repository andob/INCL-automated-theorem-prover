use std::collections::BTreeSet;
use crate::formula::{Formula, FormulaExtras, PredicateArgument};

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

pub fn generate_missing_transitive_equalities(existing_equalities : BTreeSet<(PredicateArgument, PredicateArgument)>, extras : &FormulaExtras) -> Vec<Formula>
{
    let mut missing_equalities : BTreeSet<(PredicateArgument, PredicateArgument)> = BTreeSet::new();

    for (x, y) in &existing_equalities
    {
        for (y_prime, z) in &existing_equalities
        {
            if y==y_prime && x!=y && y!=z && x!=z
            {
                if !existing_equalities.iter().any(|(a, b)| (x==a && z==b) || (x==b && z==a)) &&
                   !missing_equalities.iter().any(|(a, b)| (x==a && z==b) || (x==b && z==a))
                {
                    missing_equalities.insert((x.clone(), z.clone()));
                }
            }
        }
    }

    return missing_equalities.into_iter()
            .map(|(x, y)| Formula::Equals(x, y, extras.clone()))
            .collect::<Vec<Formula>>();
}
