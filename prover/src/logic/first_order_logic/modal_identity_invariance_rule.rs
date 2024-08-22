use std::collections::BTreeSet;
use crate::formula::{Formula, FormulaExtras, PossibleWorld, PredicateArgument, Sign};
use crate::formula::Formula::Equals;
use crate::graph::GraphVertex;
use crate::logic::{LogicRule, LogicRuleCollection};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct IdentityInvarianceRule
{
    pub base_logic_rules : LogicRuleCollection
}

impl IdentityInvarianceRule
{
    pub fn with_base_rules(base_logic_rules : LogicRuleCollection) -> IdentityInvarianceRule
    {
        return IdentityInvarianceRule { base_logic_rules };
    }
}

impl LogicRule for IdentityInvarianceRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let original_world = node.formula.get_possible_world();
        let original_graph_vertices = factory.modality_graph.vertices.clone();

        let equalities_in_original_world = factory
            .tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .filter(|formula| formula.get_possible_world() == original_world)
            .filter_map(|formula| if let Equals(x, y, _) = formula { Some((x,y)) } else { None })
            .collect::<BTreeSet<(PredicateArgument, PredicateArgument)>>();

        if let Some(mut subtree) = self.base_logic_rules.apply(factory, node)
        {
            //modality graph was not modified. nothing to do.
            if original_graph_vertices.len() == factory.modality_graph.vertices.len() { return Some(subtree) };

            //no equalities to inherit further. nothing to do.
            if equalities_in_original_world.is_empty() { return Some(subtree) };

            let new_graph_vertices = factory.modality_graph.vertices.iter()
                .filter(|vertex| !original_graph_vertices.contains(vertex))
                .collect::<BTreeSet<&GraphVertex>>();

            let new_equality_formula_extras = |world : PossibleWorld|
                FormulaExtras { possible_world: world, is_hidden: false, sign: Sign::Plus };

            let new_equality_formulas = new_graph_vertices.iter().map(|vertex| vertex.to)
                .flat_map(|new_world| equalities_in_original_world.iter().map(move |(x, y)|
                    Equals(x.clone(), y.clone(), new_equality_formula_extras(new_world))))
                .collect::<Vec<Formula>>();

            let new_equality_nodes = new_equality_formulas.into_iter()
                .map(|formula| factory.new_node(formula))
                .collect::<Vec<ProofTreeNode>>();

            subtree.append(ProofSubtree::with_middle_vertical_nodes(new_equality_nodes));
            return Some(subtree);
        }

        return None;
    }
}
