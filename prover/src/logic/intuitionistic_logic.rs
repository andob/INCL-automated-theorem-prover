use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::Formula::{And, Atomic, Imply, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules, ModalityRef};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::ManyValuedLogicSemantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 6 and 20
pub struct IntuitionisticLogic {}
impl Logic for IntuitionisticLogic
{
    fn get_name(&self) -> LogicName { LogicName::of("IntuitionisticLogic") }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(ManyValuedLogicSemantics::with_three_values());
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or, TokenTypeID::Imply,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        let modality = Rc::new(self.get_modality());
        return LogicRuleCollection::of(vec!
        [
            Box::new(ModalLogicRules::new(modality.clone())),
            Box::new(IntuitionisticLogicRules::new(modality)),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl IntuitionisticLogic
{
    pub fn get_modality(&self) -> Modality<IntuitionisticLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| true,
            is_necessity_applicable: |_, _, _| true,
            add_missing_graph_vertices: |logic, graph|
            {
                graph.add_missing_reflexive_vertices();
                graph.add_missing_transitive_vertices();
            }
        }
    }
}

struct IntuitionisticLogicRules
{
    modality : Rc<Modality<IntuitionisticLogic>>
}

impl IntuitionisticLogicRules
{
    fn new(modality : Rc<Modality<IntuitionisticLogic>>) -> IntuitionisticLogicRules
    {
        return IntuitionisticLogicRules { modality };
    }
}

impl LogicRule for IntuitionisticLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            And(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node_with_subnode(plus_p, plus_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            And(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node(minus_p);

                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, minus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_p = p.with_sign(p.get_sign() * Plus);
                let plus_p_node = factory.new_node(plus_p);

                let plus_q = q.with_sign(q.get_sign() * Plus);
                let plus_q_node = factory.new_node(plus_q);

                return Some(ProofSubtree::with_left_right_nodes(plus_p_node, plus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_q = q.with_sign(q.get_sign() * Minus);
                let minus_q_node = factory.new_node(minus_q);

                let minus_p = p.with_sign(p.get_sign() * Minus);
                let minus_p_node = factory.new_node_with_subnode(minus_p, minus_q_node);

                return Some(ProofSubtree::with_middle_node(minus_p_node));
            }

            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let minus_p = p.with_sign(Minus);
                let plus_q = q.with_sign(Plus);
                let minus_p_or_plus_q = Or(bx!(minus_p), bx!(plus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_plus_q, &extras);
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let plus_p = p.with_sign(Plus);
                let minus_q = q.with_sign(Minus);
                let plus_p_and_minus_q = And(bx!(plus_p), bx!(minus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_minus_q, &extras);
            }

            Non(box p, extras) if extras.sign == Plus =>
            {
                let minus_p = p.with_sign(Minus);

                return self.modality.apply_necessity(factory, node, &minus_p, extras);
            }

            Non(box p, extras) if extras.sign == Minus =>
            {
                let plus_p = p.with_sign(Plus);

                return self.modality.apply_possibility(factory, node, &plus_p, &extras);
            }

            p_as_formula@Atomic(_, extras) if extras.sign == Plus =>
            {
                if self.modality.was_necessity_already_applied(factory, p_as_formula) { return None };
                return self.modality.apply_necessity(factory, node, &p_as_formula, &extras.to_formula_extras());
            }

            _ => None
        }
    }
}
