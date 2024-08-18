use std::collections::BTreeSet;
use crate::formula::Formula::{Atomic, Equals, Non};
use crate::formula::{Formula, FormulaExtras, PredicateArgument};
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct HelperQuantifierRules {}

impl LogicRule for HelperQuantifierRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Equals(x, y, _), extras) =>
            {
                if x == y
                {
                    //todo check definite existence
                    //this is an object that is not equal to self ~(x=x)? forcing contradiction by stating x=x.
                    let x_equals_x = Equals(x.clone(), x.clone(), extras.clone());
                    let x_equals_x_node = factory.new_node(x_equals_x);

                    return Some(ProofSubtree::with_middle_node(x_equals_x_node));
                }

                return None;
            }

            Equals(_, _, extras) =>
            {
                //foreach node pair (x = y, y = z), generate a transitive node x = z
                let equalities = factory.tree.get_paths_that_goes_through_node(node).into_iter()
                    .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
                    .filter(|formula| formula.get_possible_world() == extras.possible_world)
                    .filter_map(|formula| if let Equals(x, y, _)
                        = formula { Some((x.clone(), y.clone())) } else { None })
                    .collect::<BTreeSet<(PredicateArgument, PredicateArgument)>>();

                let nodes = self.generate_missing_transitive_equalities(equalities, extras).into_iter()
                    .map(|formula| factory.new_node(formula))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            Non(box p@Atomic(..), extras) =>
            {
                //foreach node pair (~P[b:x], b:x = a), generate a possible contradictory node ~P[a]
                let nodes = self.generate_potentially_contradictory_atomics(factory, node, p, extras).into_iter()
                    .map(|formula| factory.new_node(formula))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            p@Atomic(_, extras) =>
            {
                //foreach node pair (P[b:x], b:x = a), generate a possible contradictory node ~[a]
                let nodes = self.generate_potentially_contradictory_atomics(factory, node, p, &extras.to_formula_extras()).into_iter()
                    .map(|formula| factory.new_node(Non(Box::new(formula), extras.to_formula_extras())))
                    .collect::<Vec<ProofTreeNode>>();

                return Some(ProofSubtree::with_middle_vertical_nodes(nodes));
            }

            _ => None,
        }
    }
}

impl HelperQuantifierRules
{
    fn generate_missing_transitive_equalities(&self, existing_equalities : BTreeSet<(PredicateArgument, PredicateArgument)>, extras : &FormulaExtras) -> Vec<Formula>
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
            .map(|(x, y)| Equals(x, y, extras.clone()))
            .collect::<Vec<Formula>>();
    }

    fn generate_potentially_contradictory_atomics(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        p : &Formula, extras : &FormulaExtras,
    ) -> Vec<Formula>
    {
        let mut formulas : Vec<Formula> = vec![];

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

        let predicate_args = p.get_predicate_arguments_of_atomic().unwrap();
        for predicate_arg in predicate_args.iter()
        {
            if predicate_arg.is_instantiated()
            {
                //todo check definite existence
                let equivalences = all_formulas_on_path.iter()
                    .filter(|formula| formula.get_possible_world() == extras.possible_world)
                    .filter_map(|formula| if let Equals(x, y, _) = formula { Some((x,y)) } else { None })
                    .filter(|(x, y)| *x==predicate_arg && free_args.contains(y))
                    .collect::<BTreeSet<(&PredicateArgument, &PredicateArgument)>>();

                for (_, equivalent_predicate_arg) in equivalences
                {
                    let object_name = equivalent_predicate_arg.object_name.clone();
                    let binded_p = p.binded(predicate_arg, object_name, extras);
                    formulas.push(binded_p);
                }
            }
        }

        return formulas;
    }
}