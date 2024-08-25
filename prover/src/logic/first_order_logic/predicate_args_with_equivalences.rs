use std::collections::BTreeSet;
use crate::formula::{Formula, PossibleWorld, PredicateArgument, PredicateArguments};
use crate::formula::Formula::Equals;
use crate::logic::first_order_logic::FirstOrderLogicDomainType::ConstantDomain;
use crate::logic::first_order_logic::variable_domain_semantics::get_args_that_definitely_exists;
use crate::tree::path::ProofTreePath;

impl Formula
{
    pub fn get_predicate_arguments_of_atomic_with_equivalences(&self, path : &ProofTreePath) -> Option<Vec<BTreeSet<PredicateArgument>>>
    {
        return self.get_predicate_arguments_of_atomic().map(|args|
            args.with_equivalences(path, self.get_possible_world()));
    }
}

impl PredicateArguments
{
    pub fn with_equivalences(self, path : &ProofTreePath, possible_world : PossibleWorld) -> Vec<BTreeSet<PredicateArgument>>
    {
        if self.is_empty() { return vec![] };

        let all_formulas_on_path = path.nodes.clone().into_iter()
            .map(|node| node.formula).collect::<Vec<Formula>>();

        let all_equivalences_on_path = all_formulas_on_path.iter()
            .filter_map(|formula| if let Equals(x, y, _) = formula { Some((x, y)) } else { None })
            .collect::<BTreeSet<(&PredicateArgument, &PredicateArgument)>>();

        let args_that_definitely_exists = get_args_that_definitely_exists(&all_formulas_on_path, possible_world);

        let mut args_with_equivalences: Vec<BTreeSet<PredicateArgument>> = vec![];

        for x in self.iter()
        {
            let mut x_equivalence_set: BTreeSet<PredicateArgument> = BTreeSet::new();
            x_equivalence_set.insert(x.clone());

            let mut equivalent_ys = all_equivalences_on_path.iter()
                .filter(|(y, z)| x==*y || x==*z)
                .map(|(y, z)| if x==*y { (*z).clone() } else { (*y).clone() })
                .filter(|a| path.domain_type == ConstantDomain ||
                    args_that_definitely_exists.iter().any(|d| *d==a))
                .collect::<BTreeSet<PredicateArgument>>();
            x_equivalence_set.append(&mut equivalent_ys);

            args_with_equivalences.push(x_equivalence_set);
        }

        return args_with_equivalences;
    }
}
