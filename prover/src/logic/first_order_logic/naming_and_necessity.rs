use std::collections::BTreeSet;
use crate::formula::{Formula, FormulaExtras, PossibleWorld, PredicateArgument, PredicateArguments};
use crate::logic::{LogicRule, LogicRuleCollection};
use crate::logic::first_order_logic::exists_quantifier_rule::ExistsQuantifierRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::problem::Problem;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 16
pub struct NonRigidDesignatorRules
{
    base_rules : LogicRuleCollection
}

impl NonRigidDesignatorRules
{
    pub fn wrap(base_rules : LogicRuleCollection) -> NonRigidDesignatorRules
    {
        return NonRigidDesignatorRules { base_rules };
    }
}

impl LogicRule for NonRigidDesignatorRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if factory.problem_flags.non_rigid_designators.is_empty()
        {
            return self.base_rules.apply(factory, node);
        }

        let old_possible_worlds = factory.modality_graph.nodes()
            .cloned().collect::<BTreeSet<PossibleWorld>>();

        if let Some(mut result_subtree) = self.base_rules.apply(factory, node)
        {
            let new_possible_worlds = factory.modality_graph.nodes().cloned()
                .filter(|possible_world| !old_possible_worlds.contains(possible_world))
                .collect::<BTreeSet<PossibleWorld>>();

            let mut new_equality_nodes = Vec::<ProofTreeNode>::new();
            for new_possible_world in new_possible_worlds
            {
                for non_rigid_designator in factory.problem_flags.non_rigid_designators.iter()
                {
                    let variable_name_factory = ExistsQuantifierRule{}.get_object_name_factory2(factory, node, &new_equality_nodes);
                    let new_variable = PredicateArgument::new_variable((*variable_name_factory)());
                    let extras = FormulaExtras::empty().in_world(new_possible_world);
                    let equality = Formula::Equals(non_rigid_designator.clone(), new_variable, extras);
                    new_equality_nodes.push(factory.new_node(equality));
                }
            }

            if !new_equality_nodes.is_empty()
            {
                result_subtree.append(&ProofSubtree::with_middle_vertical_nodes(new_equality_nodes));
            }
            
            return Some(result_subtree);
        }

        return None;
    }
}

impl Problem
{
    pub fn find_all_non_rigid_designators(&self) -> BTreeSet<PredicateArgument>
    {
        return self.premises.clone().into_iter().chain(Some(self.conclusion.clone()))
            .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
            .filter(|predicate_arg| !predicate_arg.is_rigid_designator)
            .collect::<BTreeSet<PredicateArgument>>();
    }
}
