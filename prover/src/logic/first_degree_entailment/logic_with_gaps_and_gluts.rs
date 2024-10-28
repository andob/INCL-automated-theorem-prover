use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::Formula::{And, Conditional, Non, Or};
use crate::formula::{FormulaExtras, PossibleWorld};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules, ModalityRef};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::ManyValuedLogicSemantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 9 and 23
pub struct LogicWithGapsGlutsAndWorlds
{
    is_normal : bool
}

#[allow(non_snake_case)]
impl LogicWithGapsGlutsAndWorlds
{
    pub fn K4() -> LogicWithGapsGlutsAndWorlds { LogicWithGapsGlutsAndWorlds { is_normal:true } }
    pub fn N4() -> LogicWithGapsGlutsAndWorlds { LogicWithGapsGlutsAndWorlds { is_normal:false } }
}

impl Logic for LogicWithGapsGlutsAndWorlds
{
    fn get_name(&self) -> LogicName
    {
        return if self.is_normal { LogicName::of("K4ModalLogicWithGapsAndGluts") }
        else { LogicName::of("N4ModalLogicWithGapsAndGluts") };
    }

    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(ManyValuedLogicSemantics::with_four_values());
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Necessary, TokenTypeID::Possible, TokenTypeID::Conditional,
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
            Box::new(LogicWithGapsGlutsAndWorldsConditionalRules::new(modality)),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl LogicWithGapsGlutsAndWorlds
{
    pub fn get_modality(&self) -> Modality<LogicWithGapsGlutsAndWorlds>
    {
        let is_modality_applicable = self.create_is_modality_applicable_lambda();

        return Modality
        {
            is_possibility_applicable: is_modality_applicable,
            is_necessity_applicable: is_modality_applicable,
            add_missing_graph_vertices: |logic, graph|
            {
                graph.add_missing_reflexive_vertices();
                graph.add_missing_symmetric_vertices();
                graph.add_missing_transitive_vertices();
            }
        }
    }

    fn create_is_modality_applicable_lambda(&self) -> fn(&RuleApplyFactory, &ProofTreeNode, &FormulaExtras) -> bool
    {
        if !self.is_normal
        {
            return |_factory, _node, extras|
                extras.possible_world == PossibleWorld::zero();
        }

        return |_,_,_| true;
    }
}

struct LogicWithGapsGlutsAndWorldsConditionalRules
{
    modality : Rc<Modality<LogicWithGapsGlutsAndWorlds>>
}

impl LogicWithGapsGlutsAndWorldsConditionalRules
{
    fn new(modality : Rc<Modality<LogicWithGapsGlutsAndWorlds>>) -> LogicWithGapsGlutsAndWorldsConditionalRules
    {
        return LogicWithGapsGlutsAndWorldsConditionalRules { modality };
    }
}

impl LogicRule for LogicWithGapsGlutsAndWorldsConditionalRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

        return match &node.formula
        {
            Conditional(box p, box q, extras) if extras.sign == Plus =>
            {
                let minus_p = p.with_sign(Minus);
                let plus_q = q.with_sign(Plus);
                let minus_p_or_plus_q = Or(bx!(minus_p), bx!(plus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_plus_q, extras);
            }

            Conditional(box p, box q, extras) if extras.sign == Minus =>
            {
                let plus_p = p.with_sign(Plus);
                let minus_q = q.with_sign(Minus);
                let plus_p_and_minus_q = And(bx!(plus_p), bx!(minus_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_minus_q, extras);
            }

            Non(box Conditional(box p, box q, _), extras) if extras.sign == Plus =>
            {
                let plus_p = p.with_sign(Plus);
                let plus_non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let plus_p_and_plus_non_q = And(bx!(plus_p), bx!(plus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_possibility(factory, node, &plus_p_and_plus_non_q, extras);
            }

            Non(box Conditional(box p, box q, _), extras) if extras.sign == Minus =>
            {
                let minus_p = p.with_sign(Minus);
                let minus_non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Minus);
                let minus_p_or_minus_non_q = Or(bx!(minus_p), bx!(minus_non_q), extras.with_sign(Plus).with_is_hidden(true));

                return self.modality.apply_necessity(factory, node, &minus_p_or_minus_non_q, extras);
            }

            _ => None
        }
    }
}
