use crate::formula::Formula::{Exists, ForAll, Non};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{LogicRule, LogicRuleCollection};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 6 and 20
pub struct IntuitionisticQuantifierRules
{
    base_rules : LogicRuleCollection
}

impl IntuitionisticQuantifierRules
{
    pub fn wrap(base_rules : LogicRuleCollection) -> IntuitionisticQuantifierRules
    {
        return IntuitionisticQuantifierRules { base_rules };
    }
}

impl LogicRule for IntuitionisticQuantifierRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        let modality = factory.get_logic().get_modality_ref().unwrap();
        modality.initialize_graph_if_needed(factory);

        return match &node.formula
        {
            ForAll(_, _, extras) if extras.sign == Plus =>
            {
                if node.spawner_node_id.is_none()
                {
                    return modality.apply_necessity(factory, node, &node.formula, extras)
                            .map(|subtree| subtree.with_hidden_nodes());
                }

                if let Some(spawner_node_id) = node.spawner_node_id &&
                    let Some(spawner_node) = factory.tree.get_node_with_id(spawner_node_id) &&
                    spawner_node.formula.with_stripped_extras() != node.formula.with_stripped_extras()
                {
                    return modality.apply_necessity(factory, node, &node.formula, extras)
                            .map(|subtree| subtree.with_hidden_nodes());
                }

                return self.base_rules.apply(factory, node);
            }

            ForAll(_, _, extras) if extras.sign == Minus =>
            {
                if node.spawner_node_id.is_none()
                {
                    return modality.apply_possibility(factory, node, &node.formula, extras)
                            .map(|subtree| subtree.with_hidden_nodes());
                }

                if let Some(spawner_node_id) = node.spawner_node_id &&
                    let Some(spawner_node) = factory.tree.get_node_with_id(spawner_node_id) &&
                    spawner_node.formula.with_stripped_extras() != node.formula.with_stripped_extras()
                {
                    return modality.apply_possibility(factory, node, &node.formula, extras)
                            .map(|subtree| subtree.with_hidden_nodes());
                }

                return self.base_rules.apply(factory, node);
            }

            _ =>
            {
                return self.base_rules.apply(factory, node);
            }
        }
    }
}
