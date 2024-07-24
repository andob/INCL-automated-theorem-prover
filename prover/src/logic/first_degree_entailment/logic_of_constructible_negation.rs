use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::Formula::{And, Atomic, Imply, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::first_degree_entailment::generic_biimply_fde_rule::GenericBiImplyAsConjunctionRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::three_valued_logic_semantics::{ThreeValuedContradictionBehaviour, ThreeValuedLogicSemantics};
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapter 9
pub struct LogicOfConstructibleNegation
{
    name : LogicName
}

#[allow(non_snake_case)]
impl LogicOfConstructibleNegation
{
    pub fn I4() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { name:LogicName::I4LogicOfConstructibleNegation } }
    pub fn I3() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { name:LogicName::I3LogicOfConstructibleNegation } }
    pub fn W() -> LogicOfConstructibleNegation { LogicOfConstructibleNegation { name:LogicName::WLogicOfConstructibleNegation } }
}

impl Logic for LogicOfConstructibleNegation
{
    fn get_name(&self) -> LogicName { self.name }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ThreeValuedLogicSemantics::new();

        if self.name == LogicName::I3LogicOfConstructibleNegation
        {
            semantics.add_behaviour(ThreeValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus);
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

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        let modality = Rc::new(self.get_modality());
        return vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {}),
            Box::new(ModalLogicRules::new(modality.clone())),
            Box::new(LogicOfConstructibleNegationImplicationRules::new(self.name, modality)),
            Box::new(GenericBiImplyAsConjunctionRule {}),
        ]
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
    logic_name : LogicName,
    modality : Rc<Modality<LogicOfConstructibleNegation>>
}

impl LogicOfConstructibleNegationImplicationRules
{
    fn new(logic_name : LogicName, modality : Rc<Modality<LogicOfConstructibleNegation>>) -> LogicOfConstructibleNegationImplicationRules
    {
        return LogicOfConstructibleNegationImplicationRules { logic_name, modality };
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
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let minus_p = p.with_sign(Minus);
                let plus_q = q.with_sign(Plus);
                let minus_p_or_plus_q = Or(bx!(minus_p), bx!(plus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_plus_q, extras);
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let plus_p = p.with_sign(Plus);
                let minus_q = q.with_sign(Minus);
                let plus_p_and_minus_q = And(bx!(plus_p), bx!(minus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_minus_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if extras.sign == Plus && self.logic_name != LogicName::WLogicOfConstructibleNegation =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let plus_p = p.with_sign(Plus);
                let plus_non_q = Non(bx!(q), extras.clone()).with_sign(Plus);
                let plus_p_and_plus_non_q = And(bx!(plus_p), bx!(plus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_plus_non_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if extras.sign == Minus && self.logic_name != LogicName::WLogicOfConstructibleNegation =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let minus_p = p.with_sign(Minus);
                let minus_non_q = Non(bx!(q), extras.clone()).with_sign(Minus);
                let minus_p_or_minus_non_q = Or(bx!(minus_p), bx!(minus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_minus_non_q, extras);
            }

            Non(box Imply(box p, box q, _), extras)
            if self.logic_name == LogicName::WLogicOfConstructibleNegation =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);
                let non_q = Non(bx!(q), extras.clone());
                let p_imply_non_q = Imply(bx!(p), bx!(non_q), extras.clone());
                let p_imply_non_q_node = factory.new_node(p_imply_non_q);

                return Some(ProofSubtree::with_middle_node(p_imply_non_q_node));
            }

            formula @Atomic(_, extras) if extras.sign == Plus =>
            {
                //this guard prevents infinite reapplication of □P
                if factory.modality_graph.necessity_reapplications.iter()
                    .any(|reapplication| reapplication.input_formula == *formula)
                    { return None; }

                let extras = extras.to_formula_extras();
                return self.modality.apply_necessity(factory, node, &formula, &extras);
            }

            formula@Non(box Atomic(..), extras) if extras.sign == Plus =>
            {
                //this guard prevents infinite reapplication of □!P
                if factory.modality_graph.necessity_reapplications.iter()
                    .any(|reapplication| reapplication.input_formula == *formula)
                    { return None; }

                return self.modality.apply_necessity(factory, node, &formula, &extras);
            }

            _ => None
        }
    }
}
