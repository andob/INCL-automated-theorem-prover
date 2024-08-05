use std::collections::BTreeSet;
use itertools::Itertools;
use crate::formula::{Formula, FormulaExtras, PredicateArgument};
use crate::logic::first_order_logic::exists::apply_existential_quantification;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub fn apply_for_all_quantification(
    factory : &mut RuleApplyFactory, node : &ProofTreeNode,
    x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
) -> Option<ProofSubtree>
{
    let mut output_nodes : Vec<ProofTreeNode> = vec![];

    apply_for_all_quantification_impl(factory, node, x, p, extras, &mut output_nodes);

    if output_nodes.is_empty()
    {
        return apply_existential_quantification(factory, node, x, p, extras);
    }

    return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
}

fn apply_for_all_quantification_impl(
    factory : &mut RuleApplyFactory, node : &ProofTreeNode,
    x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
    output_nodes : &mut Vec<ProofTreeNode>,
)
{
    let all_formulas_on_path = factory.tree.get_paths_that_goes_through_node(node).into_iter()
        .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
        .collect::<Vec<Formula>>();

    let all_args_on_path = all_formulas_on_path.iter()
        .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
        .collect::<BTreeSet<PredicateArgument>>();

    let free_args = get_free_args(&all_formulas_on_path, &all_args_on_path);

    let instantiated_xs = all_args_on_path.iter()
        .filter(|a| a.is_instantiated() && a.variable_name == x.variable_name)
        .collect::<BTreeSet<&PredicateArgument>>();
    for instantiated_x in instantiated_xs
    {
        let binded_p = p.binded(x, instantiated_x.object_name.clone(), extras);
        let binded_p_node = factory.new_node(binded_p);
        output_nodes.push(binded_p_node);
    }

    let equivalences = all_formulas_on_path.iter().filter_map(|formula|
        if let Formula::Equals(x, y, _) = formula { Some((x,y)) } else { None }
    ).collect::<BTreeSet<(&PredicateArgument, &PredicateArgument)>>();

    let has_no_equivalent = |x : &PredicateArgument|
        !equivalences.iter().any(|(y, z)| x==*y || x==*z);

    if has_no_equivalent(x)
    {
        let ys_with_no_equivalent = all_args_on_path.iter()
            .filter(|y| x!=*y && !y.is_instantiated() && has_no_equivalent(y))
            .collect::<BTreeSet<&PredicateArgument>>();
        for y in ys_with_no_equivalent
        {
            let instantiated_ys = all_args_on_path.iter()
                .filter(|a| a.variable_name == y.variable_name)
                .filter(|a| free_args.contains(a) || a.is_instantiated())
                .collect::<BTreeSet<&PredicateArgument>>();
            for instantiated_y in instantiated_ys
            {
                let binded_p = p.binded(x, instantiated_y.object_name.clone(), extras);
                let binded_p_node = factory.new_node(binded_p);
                output_nodes.push(binded_p_node);
            }
        }
    }
    else
    {
        //todo iterate equivalent types
        //todo don't forget about equality transitivity
    }
}

fn get_free_args(
    all_formulas_on_path : &Vec<Formula>,
    all_args_on_path : &BTreeSet<PredicateArgument>,
) -> BTreeSet<PredicateArgument>
{
    let does_not_appear_in_exists_or_for_all = |y : &PredicateArgument|
        !all_formulas_on_path.iter().any(|formula|
        {
            match formula
            {
                Formula::Exists(x, _, _) => { x.variable_name == y.variable_name }
                Formula::ForAll(x, _, _) => { x.variable_name == y.variable_name }
                _ => { false }
            }
        });

    return all_args_on_path.iter()
        .filter(|y| does_not_appear_in_exists_or_for_all(y))
        .map(|y| y.clone()).collect();
}
