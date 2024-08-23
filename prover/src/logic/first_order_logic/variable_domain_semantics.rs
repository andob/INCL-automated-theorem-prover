use std::collections::BTreeSet;
use itertools::Itertools;
use crate::formula::{Formula, FormulaExtras, PossibleWorld, PredicateArgument, Sign};
use crate::formula::Formula::{DefinitelyExists, Equals};
use crate::graph::GraphVertex;
use crate::logic::{LogicRule, LogicRuleCollection};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::path::ProofTreePath;
use crate::tree::subtree::ProofSubtree;

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
            .any(|a| args_that_definitely_exists.iter().any(|d| **d==a)); //todo shall I use .all or .any here?
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

pub struct DefinitelyExistingArgsInheritanceRule
{
    pub base_logic_rules : LogicRuleCollection
}

impl DefinitelyExistingArgsInheritanceRule
{
    pub fn with_base_rules(base_logic_rules : LogicRuleCollection) -> DefinitelyExistingArgsInheritanceRule
    {
        return DefinitelyExistingArgsInheritanceRule { base_logic_rules };
    }
}

impl LogicRule for DefinitelyExistingArgsInheritanceRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let original_world = node.formula.get_possible_world();
        let original_graph_vertices = factory.modality_graph.vertices.clone();

        let all_formulas_on_path = factory.tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .collect::<Vec<Formula>>();

        let args_that_definitely_exists = get_args_that_definitely_exists(&all_formulas_on_path, original_world);

        if let Some(mut subtree) = self.base_logic_rules.apply(factory, node)
        {
            //modality graph was not modified. nothing to do.
            if original_graph_vertices.len() == factory.modality_graph.vertices.len() { return Some(subtree) };

            //no equalities to inherit further. nothing to do.
            if args_that_definitely_exists.is_empty() { return Some(subtree) };

            let new_graph_vertices = factory.modality_graph.vertices.iter()
                .filter(|vertex| !original_graph_vertices.contains(*vertex))
                .collect::<BTreeSet<&GraphVertex>>();

            let new_definite_existence_formula_extras = |world : PossibleWorld|
                FormulaExtras { possible_world: world, is_hidden: false, sign: Sign::Plus };

            let new_definite_existence_formulas = new_graph_vertices.iter().map(|vertex| vertex.to)
                .flat_map(|new_world| args_that_definitely_exists.iter().map(move |a|
                    DefinitelyExists((*a).clone(), new_definite_existence_formula_extras(new_world))))
                .collect::<Vec<Formula>>();

            let new_definite_existence_nodes = new_definite_existence_formulas.into_iter()
                .map(|formula| factory.new_node(formula))
                .collect::<Vec<ProofTreeNode>>();

            subtree.append(ProofSubtree::with_middle_vertical_nodes(new_definite_existence_nodes));
            return Some(subtree);
        }

        return None;
    }
}
