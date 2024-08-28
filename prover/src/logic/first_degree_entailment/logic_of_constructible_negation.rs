use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::Formula::{And, Atomic, Imply, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules, ModalityRef};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::first_degree_entailment::generic_biimply_fde_rule::GenericBiImplyAsConjunctionRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::{ManyValuedContradictionBehaviour, ManyValuedLogicSemantics};
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 9 and 23
pub struct LogicOfConstructibleNegation
{
    variant : LogicOfConstructibleNegationVariant
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum LogicOfConstructibleNegationVariant { I4, I3, W }
impl LogicOfConstructibleNegationVariant
{
    fn to_logic_name(self) -> LogicName
    {
        return match self
        {
            LogicOfConstructibleNegationVariant::I4 => LogicName::of("I4LogicOfConstructibleNegation"),
            LogicOfConstructibleNegationVariant::I3 => LogicName::of("I3LogicOfConstructibleNegation"),
            LogicOfConstructibleNegationVariant::W => LogicName::of("WLogicOfConstructibleNegation")
        }
    }
}

#[allow(non_snake_case)]
impl LogicOfConstructibleNegation
{
    pub fn I4() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { variant:LogicOfConstructibleNegationVariant::I4 } }
    pub fn I3() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { variant:LogicOfConstructibleNegationVariant::I3 } }
    pub fn W() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { variant:LogicOfConstructibleNegationVariant::W } }
}

impl Logic for LogicOfConstructibleNegation
{
    fn get_name(&self) -> LogicName { self.variant.to_logic_name() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ManyValuedLogicSemantics::new();

        if self.variant == LogicOfConstructibleNegationVariant::I3
        {
            semantics.add_behaviour(ManyValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus);
        }

        return Box::new(semantics);
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis,
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        let modality = Rc::new(self.get_modality());
        return LogicRuleCollection::of(vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {}),
            Box::new(ModalLogicRules::new(modality.clone())),
            Box::new(LogicOfConstructibleNegationImplicationRules::new(self.variant, modality)),
            Box::new(GenericBiImplyAsConjunctionRule {}),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl LogicOfConstructibleNegation
{
    pub fn get_modality(&self) -> Modality<LogicOfConstructibleNegation>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| true,
            is_necessity_applicable: |_, _, _| true,
            add_missing_graph_vertices: |logic, graph|
            {
                graph.add_missing_reflexive_vertices();
                graph.add_missing_symmetric_vertices();
                graph.add_missing_transitive_vertices();
            }
        }
    }
}

struct LogicOfConstructibleNegationImplicationRules
{
    logic_variant : LogicOfConstructibleNegationVariant,
    modality : Rc<Modality<LogicOfConstructibleNegation>>
}

impl LogicOfConstructibleNegationImplicationRules
{
    fn new(logic_variant : LogicOfConstructibleNegationVariant, modality : Rc<Modality<LogicOfConstructibleNegation>>) -> LogicOfConstructibleNegationImplicationRules
    {
        return LogicOfConstructibleNegationImplicationRules { logic_variant, modality };
    }
}

impl LogicRule for LogicOfConstructibleNegationImplicationRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

        return match &node.formula
        {
            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let minus_p = p.with_sign(Minus);
                let plus_q = q.with_sign(Plus);
                let minus_p_or_plus_q = Or(bx!(minus_p), bx!(plus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_plus_q, extras);
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let plus_p = p.with_sign(Plus);
                let minus_q = q.with_sign(Minus);
                let plus_p_and_minus_q = And(bx!(plus_p), bx!(minus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_minus_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if extras.sign == Plus && self.logic_variant != LogicOfConstructibleNegationVariant::W =>
            {
                let plus_p = p.with_sign(Plus);
                let plus_non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let plus_p_and_plus_non_q = And(bx!(plus_p), bx!(plus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_plus_non_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if extras.sign == Minus && self.logic_variant != LogicOfConstructibleNegationVariant::W =>
            {
                let minus_p = p.with_sign(Minus);
                let minus_non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Minus);
                let minus_p_or_minus_non_q = Or(bx!(minus_p), bx!(minus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_minus_non_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if self.logic_variant == LogicOfConstructibleNegationVariant::W =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone());
                let p_imply_non_q = Imply(bx!(p.clone()), bx!(non_q), extras.clone());
                let p_imply_non_q_node = factory.new_node(p_imply_non_q);

                return Some(ProofSubtree::with_middle_node(p_imply_non_q_node));
            }

            p_as_formula@Atomic(_, extras) if extras.sign == Plus =>
            {
                if self.modality.was_necessity_already_applied(factory, p_as_formula) { return None };
                return self.modality.apply_necessity(factory, node, &p_as_formula, &extras.to_formula_extras());
            }

            non_p_as_formula@Non(box Atomic(_, _), extras) if extras.sign == Plus =>
            {
                if self.modality.was_necessity_already_applied(factory, non_p_as_formula) { return None };
                return self.modality.apply_necessity(factory, node, &non_p_as_formula, &extras);
            }

            _ => None
        }
    }
}
