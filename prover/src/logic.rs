use std::any::{Any, TypeId};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use anyhow::{Context, Result};
use box_macro::bx;
use strum::IntoEnumIterator;
use crate::logic::common_modal_logic::ModalityRef;
use crate::logic::conditional_modal_logic::ConditionalModalLogic;
use crate::logic::first_degree_entailment::kleene_modal_logic::KleeneModalLogic;
use crate::logic::first_degree_entailment::logic_of_constructible_negation::LogicOfConstructibleNegation;
use crate::logic::first_degree_entailment::logic_with_gaps_and_gluts::LogicWithGapsGlutsAndWorlds;
use crate::logic::first_degree_entailment::lukasiewicz_modal_logic::LukasiewiczModalLogic;
use crate::logic::first_degree_entailment::MinimalFirstDegreeEntailmentLogic;
use crate::logic::first_degree_entailment::priest_logic_of_paradox::PriestLPModalLogic;
use crate::logic::first_degree_entailment::rmingle3_modal_logic::RMingle3ModalLogic;
use crate::logic::first_order_logic::{FirstOrderLogicDomainType, FirstOrderLogicIdentityType, FirstOrderLogic, FIRST_ORDER_LOGIC_NAME_PREFIX};
use crate::logic::first_order_logic::FirstOrderLogicDomainType::{ConstantDomain, VariableDomain};
use crate::logic::first_order_logic::FirstOrderLogicIdentityType::{ContingentIdentity, NecessaryIdentity};
use crate::logic::fuzzy_logic::LukasiewiczFuzzyLogic;
use crate::logic::intuitionistic_logic::IntuitionisticLogic;
use crate::logic::non_normal_modal_logic::NonNormalModalLogic;
use crate::logic::normal_modal_logic::NormalModalLogic;
use crate::logic::propositional_logic::PropositionalLogic;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::logic::temporal_modal_logic::TemporalModalLogic;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub mod propositional_logic;
pub mod first_order_logic;
mod normal_modal_logic;
mod non_normal_modal_logic;
pub mod common_modal_logic;
pub mod rule_apply_factory;
pub mod intuitionistic_logic;
mod temporal_modal_logic;
mod conditional_modal_logic;
mod first_degree_entailment;
mod fuzzy_logic;

pub trait Logic : Any
{
    //the logic name, eg: PropositionalLogic
    fn get_name(&self) -> LogicName;

    //cast to &dyn Any (kindof a void* pointer)
    fn as_any(&self) -> &dyn Any;

    //logic semantics, eg: true/false, true/false/unknown
    fn get_semantics(&self) -> Box<dyn Semantics>;

    //list of syntactic symbols, eg: not, and, or
    fn get_parser_syntax(&self) -> Vec<TokenTypeID>;

    //tree decomposition rules
    fn get_rules(&self) -> LogicRuleCollection;

    //a reference to the modality or None if the logic is not modal logic
    //cannot use Option<Modality<LOGIC>> because trait functions cannot return generic types :(
    fn get_modality_ref(&self) -> Option<ModalityRef>;
}

impl dyn Logic
{
    #[inline]
    pub fn cast_to<OutputLogic : Logic>(&self) -> Option<&OutputLogic>
    {
        if self.get_name().is_first_order_logic() &&
           TypeId::of::<OutputLogic>() != TypeId::of::<FirstOrderLogic>()
        {
            //we're looking for something other than FirstOrderLogic, so we'll check the base logic
            let first_order_logic = self.as_any().downcast_ref::<FirstOrderLogic>()?;
            return first_order_logic.base_logic.cast_to::<OutputLogic>();
        }

        return self.as_any().downcast_ref::<OutputLogic>();
    }
}

pub trait LogicRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>;
}

pub struct LogicRuleCollection
{
    rules : Vec<Box<dyn LogicRule>>
}

impl LogicRuleCollection
{
    pub fn of(rules : Vec<Box<dyn LogicRule>>) -> LogicRuleCollection
    {
        return LogicRuleCollection { rules };
    }

    pub fn append(&mut self, another : &mut LogicRuleCollection)
    {
        self.rules.append(&mut another.rules);
    }

    pub fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        for logic_rule in &self.rules
        {
            if let Some(subtree) = logic_rule.apply(factory, node)
            {
                return Some(subtree);
            }
        }

        return None;
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct LogicName { value : String }

impl LogicName
{
    pub fn of(value: &str) -> LogicName
    {
        return LogicName { value:String::from(value) }
    }
}

impl Display for LogicName
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", self.value);
    }
}

impl LogicName
{
    pub fn is_modal_logic(&self) -> bool
    {
        return !self.matches_name_of_logic(bx!(PropositionalLogic{})) &&
            !self.matches_name_of_logic(bx!(MinimalFirstDegreeEntailmentLogic{})) &&
            !self.matches_name_of_logic(bx!(LukasiewiczFuzzyLogic{}));
    }

    pub fn is_non_normal_modal_logic(&self) -> bool
    {
        return self.matches_name_of_logic(bx!(NonNormalModalLogic::N())) ||
            self.matches_name_of_logic(bx!(NonNormalModalLogic::S2())) ||
            self.matches_name_of_logic(bx!(NonNormalModalLogic::S3())) ||
            self.matches_name_of_logic(bx!(NonNormalModalLogic::S3_5())) ||
            self.matches_name_of_logic(bx!(LogicWithGapsGlutsAndWorlds::N4()));
    }

    pub fn is_intuitionistic_logic(&self) -> bool
    {
        return self.matches_name_of_logic(bx!(IntuitionisticLogic{})) ||
            self.matches_name_of_logic(bx!(LogicOfConstructibleNegation::I4())) ||
            self.matches_name_of_logic(bx!(LogicOfConstructibleNegation::I3())) ||
            self.matches_name_of_logic(bx!(LogicOfConstructibleNegation::W()));
    }

    fn matches_name_of_logic(&self, logic : Box<dyn Logic>) -> bool
    {
        let target_value = logic.get_name().to_string();

        //not using a regex for efficiency
        return self.value == target_value ||
            self.value.ends_with(format!("+{}", target_value).as_str()) ||
            self.value.contains(format!("+{}+", target_value).as_str()) ||
            self.value.starts_with(format!("{}+", target_value).as_str());
    }

    pub fn is_normal_modal_logic(&self) -> bool
    {
        return self.is_modal_logic() && !self.is_non_normal_modal_logic();
    }

    pub fn is_first_order_logic(&self) -> bool
    {
        return self.value.starts_with(FIRST_ORDER_LOGIC_NAME_PREFIX);
    }
}

pub struct LogicFactory {}
impl LogicFactory
{
    pub fn get_logic_by_name(name : &String) -> Result<Rc<dyn Logic>>
    {
        return Self::get_logic_theories().into_iter()
            .find(|logic| logic.get_name().to_string().as_str() == name.as_str())
            .context(format!("Invalid logic with name {}!", name));
    }

    pub fn get_logic_theories() -> Vec<Rc<dyn Logic>>
    {
        let base_logics : Vec<Rc<dyn Logic>> = vec!
        [
            Rc::new(PropositionalLogic {}),

            Rc::new(NormalModalLogic::K()),
            Rc::new(NormalModalLogic::T()),
            Rc::new(NormalModalLogic::B()),
            Rc::new(NormalModalLogic::S4()),
            Rc::new(NormalModalLogic::S5()),

            Rc::new(NonNormalModalLogic::S0_5()),
            Rc::new(NonNormalModalLogic::N()),
            Rc::new(NonNormalModalLogic::S2()),
            Rc::new(NonNormalModalLogic::S3()),
            Rc::new(NonNormalModalLogic::S3_5()),

            Rc::new(TemporalModalLogic::basic()),
            Rc::new(TemporalModalLogic::extended()),

            Rc::new(ConditionalModalLogic::basic()),
            Rc::new(ConditionalModalLogic::extended()),

            Rc::new(IntuitionisticLogic{}),

            Rc::new(MinimalFirstDegreeEntailmentLogic {}),

            Rc::new(LukasiewiczModalLogic::L3_K()),
            Rc::new(LukasiewiczModalLogic::L3_T()),
            Rc::new(LukasiewiczModalLogic::L3_B()),
            Rc::new(LukasiewiczModalLogic::L3_S4()),
            Rc::new(LukasiewiczModalLogic::L3_S5()),

            Rc::new(RMingle3ModalLogic::RM3_K()),
            Rc::new(RMingle3ModalLogic::RM3_T()),
            Rc::new(RMingle3ModalLogic::RM3_B()),
            Rc::new(RMingle3ModalLogic::RM3_S4()),
            Rc::new(RMingle3ModalLogic::RM3_S5()),

            Rc::new(PriestLPModalLogic::LP_K()),
            Rc::new(PriestLPModalLogic::LP_T()),
            Rc::new(PriestLPModalLogic::LP_B()),
            Rc::new(PriestLPModalLogic::LP_S4()),
            Rc::new(PriestLPModalLogic::LP_S5()),

            Rc::new(KleeneModalLogic::K3_K()),
            Rc::new(KleeneModalLogic::K3_T()),
            Rc::new(KleeneModalLogic::K3_B()),
            Rc::new(KleeneModalLogic::K3_S4()),
            Rc::new(KleeneModalLogic::K3_S5()),

            Rc::new(LogicWithGapsGlutsAndWorlds::K4()),
            Rc::new(LogicWithGapsGlutsAndWorlds::N4()),

            Rc::new(LogicOfConstructibleNegation::I3()),
            Rc::new(LogicOfConstructibleNegation::I4()),
            Rc::new(LogicOfConstructibleNegation::W()),

            Rc::new(LukasiewiczFuzzyLogic {}),
        ];

        let excluded_logics = base_logics.iter()
            .filter(|logic| logic.get_name().is_intuitionistic_logic())
            .flat_map(|intuitionistic_logic| vec!
            [
                FirstOrderLogic { base_logic:intuitionistic_logic.clone(), domain_type:ConstantDomain, identity_type:NecessaryIdentity },
                FirstOrderLogic { base_logic:intuitionistic_logic.clone(), domain_type:ConstantDomain, identity_type:ContingentIdentity },
                FirstOrderLogic { base_logic:intuitionistic_logic.clone(), domain_type:VariableDomain, identity_type:NecessaryIdentity },
            ])
            .collect::<Vec<FirstOrderLogic>>();

        let mut output_logics = base_logics.clone();

        for domain_type in FirstOrderLogicDomainType::iter()
        {
            for identity_type in FirstOrderLogicIdentityType::iter()
            {
                for base_logic in &base_logics
                {
                    let first_order_logic = FirstOrderLogic
                    {
                        domain_type, identity_type,
                        base_logic: base_logic.clone()
                    };

                    if !excluded_logics.contains(&&first_order_logic)
                    {
                        output_logics.push(Rc::new(first_order_logic));
                    }
                }
            }
        }

        return output_logics;
    }
}
