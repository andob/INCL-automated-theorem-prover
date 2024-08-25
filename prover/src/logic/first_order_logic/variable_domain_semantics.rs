use std::borrow::Borrow;
use std::collections::BTreeSet;
use itertools::Itertools;
use crate::formula::{Formula, PossibleWorld, PredicateArgument};
use crate::formula::Formula::{DefinitelyExists, Equals};
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;

pub struct VariableDomainSemantics
{
    base_semantics : Box<dyn Semantics>
}

impl VariableDomainSemantics
{
    pub fn new(base_semantics : Box<dyn Semantics>) -> VariableDomainSemantics
    {
        return VariableDomainSemantics { base_semantics };
    }
}

impl Semantics for VariableDomainSemantics
{
    fn number_of_truth_values(&self) -> u8
    {
        return self.base_semantics.number_of_truth_values();
    }

    fn negate(&self, formula : &Formula) -> Formula
    {
        return self.base_semantics.negate(formula);
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        return self.base_semantics.are_formulas_contradictory(path, p, q) &&
                self.are_all_predicate_args_in_domain(path, p) &&
                self.are_all_predicate_args_in_domain(path, p);
    }
}

impl VariableDomainSemantics
{
    fn are_all_predicate_args_in_domain(&self, path : &ProofTreePath, p : &Formula) -> bool
    {
        let p_predicate_args = p.get_all_predicate_arguments();
        if p_predicate_args.is_empty() { return true };

        let all_formulas_on_path = path.nodes.clone().into_iter()
            .map(|node| node.formula).collect::<Vec<Formula>>();

        let mut args_that_definitely_exists = get_args_that_definitely_exists(&all_formulas_on_path, p.get_possible_world());

        let free_args = all_formulas_on_path.iter()
            .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
            .filter(|y| y.object_name == y.variable_name && !all_formulas_on_path.iter()
                .any(|formula| formula.contains_quantifier_with_argument(y)))
            .collect::<BTreeSet<PredicateArgument>>();

        return p_predicate_args.into_iter()
            .filter(|a| free_args.contains(a) || a.is_instantiated())
            .any(|a| args_that_definitely_exists.iter().any(|d| **d==a));
    }
}

pub fn get_args_that_definitely_exists(all_formulas_on_path : &Vec<Formula>, possible_world : PossibleWorld) -> BTreeSet<&PredicateArgument>
{
    let mut args_that_definitely_exists = all_formulas_on_path.iter()
        .filter(|formula| formula.get_possible_world() == possible_world)
        .filter_map(|formula| if let DefinitelyExists(x, _) = formula { Some(x) } else { None })
        .collect::<BTreeSet<&PredicateArgument>>();

    let mut equivalences_of_args_that_definitely_exists = all_formulas_on_path.iter()
        .filter(|formula| formula.get_possible_world() == possible_world)
        .filter_map(|formula| if let Equals(x, y, _) = formula { Some((x, y)) } else { None })
        .filter(|(x, y)| args_that_definitely_exists.iter().any(|d| x==d || y==d))
        .flat_map(|(x, y)| vec![x, y])
        .collect::<BTreeSet<&PredicateArgument>>();

    args_that_definitely_exists.append(&mut equivalences_of_args_that_definitely_exists);
    return args_that_definitely_exists;
}
