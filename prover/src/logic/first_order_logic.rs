mod exists_quantifier_rule;
mod forall_quantifier_rule;
mod helper_quantifier_rules;
mod variable_domain_semantics;
mod predicate_args_with_equivalences;
mod intuitionistic_quantifier_rules;

use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use strum_macros::{Display, EnumIter};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::ModalityRef;
use crate::logic::first_order_logic::exists_quantifier_rule::ExistsQuantifierRule;
use crate::logic::first_order_logic::forall_quantifier_rule::ForAllQuantifierRule;
use crate::logic::first_order_logic::helper_quantifier_rules::HelperQuantifierRules;
use crate::logic::first_order_logic::intuitionistic_quantifier_rules::IntuitionisticQuantifierRules;
use crate::logic::first_order_logic::variable_domain_semantics::VariableDomainSemantics;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;

//check out book chapters 12-23
pub struct FirstOrderLogic
{
    pub domain_type : FirstOrderLogicDomainType,
    pub identity_type : FirstOrderLogicIdentityType,
    pub base_logic : Rc<dyn Logic>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, EnumIter, Display)]
pub enum FirstOrderLogicDomainType
{
    ConstantDomain, VariableDomain
}

impl Default for FirstOrderLogicDomainType
{
    fn default() -> Self { FirstOrderLogicDomainType::ConstantDomain }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, EnumIter, Display)]
pub enum FirstOrderLogicIdentityType
{
    NecessaryIdentity, ContingentIdentity
}

impl Default for FirstOrderLogicIdentityType
{
    fn default() -> Self { FirstOrderLogicIdentityType::NecessaryIdentity }
}

impl Eq for FirstOrderLogic {}
impl PartialEq for FirstOrderLogic
{
    fn eq(&self, other : &Self) -> bool { self.get_name() == other.get_name() }
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
        let base_semantics = self.base_logic.get_semantics();

        if self.domain_type == FirstOrderLogicDomainType::VariableDomain
        {
            return Box::new(VariableDomainSemantics::new(base_semantics));
        }

        return base_semantics;
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        let mut syntax = vec!
        [
            TokenTypeID::AtomicWithArgs,
            TokenTypeID::Exists, TokenTypeID::ForAll, TokenTypeID::Equals,
        ];

        if self.domain_type == FirstOrderLogicDomainType::VariableDomain
        {
            syntax.push(TokenTypeID::DefinitelyExists);
        }

        syntax.append(&mut self.base_logic.get_parser_syntax());
        return syntax;
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        let mut rules : LogicRuleCollection = LogicRuleCollection::of(vec!
        [
            Box::new(ExistsQuantifierRule{}),
            Box::new(ForAllQuantifierRule{}),
            Box::new(HelperQuantifierRules{}),
        ]);

        rules.append(&mut self.base_logic.get_rules());

        if self.base_logic.get_name().is_intuitionistic_logic()
        {
            let wrapper_rule = IntuitionisticQuantifierRules::wrap(rules);
            rules = LogicRuleCollection::of(vec![bx!(wrapper_rule)]);
        }

        return rules;
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return self.base_logic.get_modality_ref();
    }
}
