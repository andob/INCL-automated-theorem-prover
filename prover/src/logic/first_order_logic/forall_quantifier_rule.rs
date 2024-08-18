use std::collections::BTreeSet;
use box_macro::bx;
use crate::formula::Formula::{Exists, ForAll, Non};
use crate::formula::{Formula, FormulaExtras, PredicateArgument};
use crate::logic::first_order_logic::exists_quantifier_rule::ExistsQuantifierRule;
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct ForAllQuantifierRule {}

impl LogicRule for ForAllQuantifierRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let Non(box ForAll(x, box p, _), extras) = &node.formula
        {
            let non_p = Non(bx!(p.clone()), extras.clone());
            let exists_non_p = Exists(x.clone(), bx!(non_p), extras.clone());
            let exists_non_p_node = factory.new_node(exists_non_p);

            return Some(ProofSubtree::with_middle_node(exists_non_p_node));
        }

        if let ForAll(x, box p, extras) = &node.formula
        {
            let mut output_nodes : Vec<ProofTreeNode> = vec![];

            self.apply_for_all_quantification_impl(factory, node, x, p, extras, &mut output_nodes);

            if output_nodes.is_empty()
            {
                //when there are no nodes, act similar to exists quantifier
                let object_name_factory = ExistsQuantifierRule::get_object_name_factory(factory, node);
                let instantiated_p = p.instantiated(x, &object_name_factory, extras);
                let instantiated_p_node = factory.new_node(instantiated_p);

                return Some(ProofSubtree::with_middle_node(instantiated_p_node));
            }

            return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
        }

        return None;
    }
}

impl ForAllQuantifierRule
{
    fn apply_for_all_quantification_impl(&self,
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

        //todo create is_free_arg closure
        let free_args = all_args_on_path.iter()
            .filter(|y| y.object_name == y.variable_name && !all_formulas_on_path.iter()
                .any(|formula| formula.contains_quantifier_with_argument(y)))
            .collect::<BTreeSet<&PredicateArgument>>();

        //todo check definite existence
        let instantiated_xs = all_args_on_path.iter()
            .filter(|a| a.is_instantiated() && a.variable_name == x.variable_name)
            .collect::<BTreeSet<&PredicateArgument>>();
        for instantiated_x in instantiated_xs
        {
            let binded_p = p.binded(x, instantiated_x.object_name.clone(), extras);
            let binded_p_node = factory.new_node(binded_p);
            output_nodes.push(binded_p_node);
        }

        let equivalences = all_formulas_on_path.iter()
            .filter(|formula| formula.get_possible_world() == extras.possible_world)
            .filter_map(|formula| if let Formula::Equals(x, y, _) = formula { Some((x,y)) } else { None })
            .collect::<BTreeSet<(&PredicateArgument, &PredicateArgument)>>();

        let has_no_equivalent = |x : &PredicateArgument| !equivalences.iter()
            .any(|(y, z)| x.variable_name == *y.variable_name || x.variable_name == *z.variable_name);

        if has_no_equivalent(x)
        {
            let ys_with_no_equivalent = all_args_on_path.iter()
                .filter(|y| !y.is_instantiated() && has_no_equivalent(y))
                .filter(|y| x.variable_name != y.variable_name)
                .collect::<BTreeSet<&PredicateArgument>>();

            for y in ys_with_no_equivalent
            {
                //todo check definite existence
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
            let equivalent_ys = equivalences.iter()
                .filter(|(y, z)| x.variable_name == *y.variable_name || x.variable_name == *z.variable_name)
                .map(|(y, z)| if x.variable_name == *y.variable_name { *z } else { *y })
                .collect::<BTreeSet<&PredicateArgument>>();

            for y in equivalent_ys
            {
                //todo check definite existence
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
    }
}
