use std::any::Any;
use crate::logic::{Logic, LogicName, LogicRuleCollection};
use crate::logic::common_modal_logic::ModalityRef;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::fuzzy_logic_semantics::FuzzyLogicSemantics;
use crate::semantics::Semantics;

pub struct LukasiewiczFuzzyLogic {}

impl Logic for LukasiewiczFuzzyLogic
{
    fn get_name(&self) -> LogicName { LogicName::of("LukasiewiczFuzzyLogic") }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(FuzzyLogicSemantics {});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec![])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef> { None }
}
