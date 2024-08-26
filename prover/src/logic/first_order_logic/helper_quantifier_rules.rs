use std::collections::BTreeSet;
use crate::formula::Formula::{DefinitelyExists, Equals, Non};
use crate::formula::{Formula, FormulaExtras, PredicateArgument};
use crate::logic::first_order_logic::{FirstOrderLogic, FirstOrderLogicDomainType, FirstOrderLogicIdentityType};
use crate::logic::first_order_logic::variable_domain_semantics::get_args_that_definitely_exists;
use crate::logic::{Logic, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct HelperQuantifierRules {}

impl LogicRule for HelperQuantifierRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let mut subtree = ProofSubtree::empty();

        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<FirstOrderLogic>()?;

        match &node.formula
        {
            p@Equals(_, _, extras) =>
            {
                //foreach node pair (x = y, y = z), generate a transitive node x = z
                subtree.append(&self.create_subtree_with_missing_transitive_nodes(factory, node, extras));

                if logic.identity_type == FirstOrderLogicIdentityType::NecessaryIdentity && logic.get_name().is_modal_logic()
                {
                    //inherit x=y to all possible worlds by stating □(x=y)
                    if let Some(modality) = logic.get_modality_ref() && !modality.was_necessity_already_applied(factory, p)
                    {
                        subtree.append(&modality.apply_necessity(factory, node, &p, extras).unwrap_or_default());
                    }
                }
            }

            non_p @Non(box Equals(x, y, _), extras) =>
            {
                if x == y
                {
                    //this is an object that is not equal to self ~(x=x)? forcing contradiction by stating x=x.
                    subtree.append(&self.create_subtree_with_x_equals_to_x_node(factory, node, x, extras));
                }

                if logic.identity_type == FirstOrderLogicIdentityType::NecessaryIdentity && logic.get_name().is_modal_logic()
                {
                    //inherit !(x=y) to all possible worlds by stating □!(x=y)
                    if let Some(modality) = logic.get_modality_ref() && !modality.was_necessity_already_applied(factory, non_p)
                    {
                        subtree.append(&modality.apply_necessity(factory, node, &non_p, extras).unwrap_or_default());
                    }
                }
            }

            p@DefinitelyExists(_, extras) =>
            {
                if logic.domain_type == FirstOrderLogicDomainType::VariableDomain && logic.get_name().is_modal_logic()
                {
                    //inherit 𝔈x to all possible worlds by stating □𝔈x
                    if let Some(modality) = logic.get_modality_ref() && !modality.was_necessity_already_applied(factory, p)
                    {
                        subtree.append(&modality.apply_necessity(factory, node, &p, extras).unwrap_or_default());
                    }
                }
            }

            non_p @Non(box DefinitelyExists(_, _), extras) =>
            {
                if logic.domain_type == FirstOrderLogicDomainType::VariableDomain && logic.get_name().is_modal_logic()
                {
                    //inherit !𝔈x to all possible worlds by stating □!𝔈x
                    if let Some(modality) = logic.get_modality_ref() && !modality.was_necessity_already_applied(factory, non_p)
                    {
                        subtree.append(&modality.apply_necessity(factory, node, &non_p, extras).unwrap_or_default());
                    }
                }
            }

            _ => {}
        }

        return if !subtree.is_empty() { Some(subtree) } else { None };
    }
}

impl HelperQuantifierRules
{
    fn create_subtree_with_x_equals_to_x_node(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, x : &PredicateArgument, extras : &FormulaExtras) -> ProofSubtree
    {
        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<FirstOrderLogic>().unwrap();

        let x_equals_x = Equals(x.clone(), x.clone(), extras.clone());
        let x_equals_x_node = factory.new_node(x_equals_x);

        if logic.domain_type == FirstOrderLogicDomainType::VariableDomain
        {
            let all_formulas_on_path = factory.tree.get_paths_that_goes_through_node(node).into_iter()
                .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
                .collect::<Vec<Formula>>();

            let args_that_definitely_exists = get_args_that_definitely_exists(&all_formulas_on_path, extras.possible_world);
            if !args_that_definitely_exists.into_iter().any(|arg| arg==x)
            {
                return ProofSubtree::empty();
            }
        }

        return ProofSubtree::with_middle_node(x_equals_x_node);
    }

    fn create_subtree_with_missing_transitive_nodes(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode, extras : &FormulaExtras) -> ProofSubtree
    {
        let equalities = factory.tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .filter(|formula| formula.get_possible_world() == extras.possible_world)
            .filter_map(|formula| if let Equals(x, y, _)
                = formula { Some((x.clone(), y.clone())) } else { None })
            .collect::<BTreeSet<(PredicateArgument, PredicateArgument)>>();

        let nodes = self.generate_missing_transitive_equalities(equalities, extras).into_iter()
            .map(|formula| factory.new_node(formula))
            .collect::<Vec<ProofTreeNode>>();

        return ProofSubtree::with_middle_vertical_nodes(nodes);
    }

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
}
