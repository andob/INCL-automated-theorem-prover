use std::any::Any;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::logic::conditional_modal_logic::ConditionalModalLogic;
use crate::logic::first_degree_entailment::kleene_modal_logic::KleeneModalLogic;
use crate::logic::first_degree_entailment::lukasiewicz_modal_logic::LukasiewiczModalLogic;
use crate::logic::first_degree_entailment::MinimalFirstDegreeEntailmentLogic;
use crate::logic::first_degree_entailment::priest_logic_of_paradox::PriestLPModalLogic;
use crate::logic::first_degree_entailment::rmingle3_modal_logic::RMingle3ModalLogic;
use crate::logic::first_order_logic::FirstOrderLogic;
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

pub trait LogicRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>;
}

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
    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>;
}

#[derive(Eq, PartialEq, Copy, Clone, EnumIter)]
pub enum LogicName
{
    PropositionalLogic, FirstOrderLogic,
    KModalLogic, TModalLogic, BModalLogic, S4ModalLogic, S5ModalLogic,
    NModalLogic, S2ModalLogic, S3ModalLogic, S3_5ModalLogic,
    KTemporalModalLogic, KTemporalExtModalLogic,
    ConditionalModalLogic, ConditionalExtModalLogic,
    IntuitionisticLogic,
    MinimalFirstDegreeEntailmentLogic,
    LukasiewiczModalLogic(usize),
    RMingle3ModalLogic(usize),
    KleeneModalLogic(usize),
    PriestLogicOfParadoxModalLogic(usize),
}

impl Display for LogicName
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return match self
        {
            LogicName::PropositionalLogic => { write!(f, "{}", "PropositionalLogic") }
            LogicName::FirstOrderLogic => { write!(f, "{}", "FirstOrderLogic") }
            LogicName::KModalLogic => { write!(f, "{}", "KModalLogic") }
            LogicName::TModalLogic => { write!(f, "{}", "TModalLogic") }
            LogicName::BModalLogic => { write!(f, "{}", "BModalLogic") }
            LogicName::S4ModalLogic => { write!(f, "{}", "S4ModalLogic") }
            LogicName::S5ModalLogic => { write!(f, "{}", "S5ModalLogic") }
            LogicName::NModalLogic => { write!(f, "{}", "NModalLogic") }
            LogicName::S2ModalLogic => { write!(f, "{}", "S2ModalLogic") }
            LogicName::S3ModalLogic => { write!(f, "{}", "S3ModalLogic") }
            LogicName::S3_5ModalLogic => { write!(f, "{}", "S3.5ModalLogic") }
            LogicName::KTemporalModalLogic => { write!(f, "{}", "KTemporalModalLogic") }
            LogicName::KTemporalExtModalLogic => { write!(f, "{}", "KTemporalExtModalLogic") }
            LogicName::ConditionalModalLogic => { write!(f, "{}", "ConditionalModalLogic") }
            LogicName::ConditionalExtModalLogic => { write!(f, "{}", "ConditionalExtModalLogic") }
            LogicName::IntuitionisticLogic => { write!(f, "{}", "IntuitionisticLogic") }
            LogicName::MinimalFirstDegreeEntailmentLogic => { write!(f, "{}", "MinimalFirstDegreeEntailmentLogic") }
            LogicName::LukasiewiczModalLogic(base_logic_name_index) =>
            {
                let base_logic_name = LogicName::iter().get(*base_logic_name_index).unwrap();
                write!(f, "Lukasiewicz+{}", base_logic_name.to_string())
            }
            LogicName::RMingle3ModalLogic(base_logic_name_index) =>
            {
                let base_logic_name = LogicName::iter().get(*base_logic_name_index).unwrap();
                write!(f, "RMingle3+{}", base_logic_name.to_string())
            }
            LogicName::KleeneModalLogic(base_logic_name_index) =>
            {
                let base_logic_name = LogicName::iter().get(*base_logic_name_index).unwrap();
                write!(f, "Kleene+{}", base_logic_name.to_string())
            }
            LogicName::PriestLogicOfParadoxModalLogic(base_logic_name_index) =>
            {
                let base_logic_name = LogicName::iter().get(*base_logic_name_index).unwrap();
                write!(f, "PriestLogicOfParadox+{}", base_logic_name.to_string())
            }
        }
    }
}

impl LogicName
{
    pub fn is_modal_logic(self) -> bool
    {
        return self != LogicName::PropositionalLogic && self != LogicName::FirstOrderLogic;
    }

    pub fn is_non_normal_modal_logic(self) -> bool
    {
        return self == LogicName::NModalLogic || self == LogicName::S2ModalLogic ||
            self == LogicName::S3_5ModalLogic || self == LogicName::S3_5ModalLogic;
    }

    pub fn is_normal_modal_logic(self) -> bool
    {
        return self.is_modal_logic() && !self.is_non_normal_modal_logic();
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
        return vec!
        [
            Rc::new(PropositionalLogic {}),
            Rc::new(FirstOrderLogic {}),

            Rc::new(NormalModalLogic::K()),
            Rc::new(NormalModalLogic::T()),
            Rc::new(NormalModalLogic::B()),
            Rc::new(NormalModalLogic::S4()),
            Rc::new(NormalModalLogic::S5()),

            Rc::new(NonNormalModalLogic::N()),
            Rc::new(NonNormalModalLogic::S2()),
            Rc::new(NonNormalModalLogic::S3()),
            Rc::new(NonNormalModalLogic::S3_5()),

            Rc::new(TemporalModalLogic::basic()),
            Rc::new(TemporalModalLogic::extended()),

            Rc::new(ConditionalModalLogic::basic()),
            Rc::new(ConditionalModalLogic::extended()),

            Rc::new(IntuitionisticLogic {}),

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
        ]
    }
}
