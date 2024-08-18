mod exists_quantifier_rule;
mod forall_quantifier_rule;
mod helper_quantifier_rules;
mod modal_identity_invariance_rule;

use std::any::Any;
use std::rc::Rc;
use strum_macros::{Display, EnumIter};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::first_order_logic::exists_quantifier_rule::ExistsQuantifierRule;
use crate::logic::first_order_logic::forall_quantifier_rule::ForAllQuantifierRule;
use crate::logic::first_order_logic::helper_quantifier_rules::HelperQuantifierRules;
use crate::logic::first_order_logic::modal_identity_invariance_rule::IdentityInvarianceRule;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;

//check out book chapters 12-23
pub struct FirstOrderLogic
{
    pub domain_type : FirstOrderLogicDomainType,
    pub identity_type : FirstOrderLogicIdentityType,
    pub base_logic : Rc<dyn Logic>,
}

#[derive(Eq, PartialEq, Copy, Clone, EnumIter, Display)]
pub enum FirstOrderLogicDomainType
{
    ConstantDomain, VariableDomain
}

impl Default for FirstOrderLogicDomainType
{
    fn default() -> Self { FirstOrderLogicDomainType::ConstantDomain }
}

#[derive(Eq, PartialEq, Copy, Clone, EnumIter, Display)]
pub enum FirstOrderLogicIdentityType
{
    NecessaryIdentity, ContingentIdentity
}

impl Default for FirstOrderLogicIdentityType
{
    fn default() -> Self { FirstOrderLogicIdentityType::NecessaryIdentity }
}

pub const FIRST_ORDER_LOGIC_NAME_PREFIX : &str = "FirstOrderLogic";

impl Logic for FirstOrderLogic
{
    fn get_name(&self) -> LogicName
    {
        return LogicName::of(format!("{}+{}+{}+{}",
            FIRST_ORDER_LOGIC_NAME_PREFIX, self.domain_type,
            self.identity_type, self.base_logic.get_name()).as_str());
    }

    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return self.base_logic.get_semantics();
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        let mut syntax = vec!
        [
            TokenTypeID::AtomicWithArgs,
            TokenTypeID::Exists, TokenTypeID::ForAll, TokenTypeID::Equals,
        ];

        syntax.append(&mut self.base_logic.get_parser_syntax());
        return syntax;
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        let mut rules : Vec<Box<dyn LogicRule>> = vec!
        [
            Box::new(ExistsQuantifierRule{}),
            Box::new(ForAllQuantifierRule{}),
            Box::new(HelperQuantifierRules{}),
        ];

        rules.append(&mut self.base_logic.get_rules().to_vec());

        if self.base_logic.get_name().is_modal_logic()
        {
            let identity_invariance_rule = IdentityInvarianceRule::with_base_rules(LogicRuleCollection::of(rules));
            return LogicRuleCollection::of(vec![ Box::new(identity_invariance_rule) ]);
        }

        return LogicRuleCollection::of(rules);
    }
}
