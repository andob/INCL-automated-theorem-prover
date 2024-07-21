use std::any::Any;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use anyhow::{Context, Result};
use crate::logic::conditional_modal_logic::ConditionalModalLogic;
use crate::logic::first_degree_entailment::MinimalFirstDegreeEntailmentLogic;
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
        ];
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum LogicName
{
    PropositionalLogic, FirstOrderLogic,
    KModalLogic, TModalLogic, BModalLogic, S4ModalLogic, S5ModalLogic,
    NModalLogic, S2ModalLogic, S3ModalLogic, S3_5ModalLogic,
    KTemporalModalLogic, KTemporalExtModalLogic,
    ConditionalModalLogic, ConditionalExtModalLogic,
    IntuitionisticLogic, MinimalFirstDegreeEntailmentLogic,
}

impl Display for LogicName
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}", match self
        {
            LogicName::PropositionalLogic => { "PropositionalLogic" }
            LogicName::FirstOrderLogic => { "FirstOrderLogic" }
            LogicName::KModalLogic => { "KModalLogic" }
            LogicName::TModalLogic => { "TModalLogic" }
            LogicName::BModalLogic => { "BModalLogic" }
            LogicName::S4ModalLogic => { "S4ModalLogic" }
            LogicName::S5ModalLogic => { "S5ModalLogic" }
            LogicName::NModalLogic => { "NModalLogic" }
            LogicName::S2ModalLogic => { "S2ModalLogic" }
            LogicName::S3ModalLogic => { "S3ModalLogic" }
            LogicName::S3_5ModalLogic => { "S3.5ModalLogic" }
            LogicName::KTemporalModalLogic => { "KTemporalModalLogic" }
            LogicName::KTemporalExtModalLogic => { "KTemporalExtModalLogic" }
            LogicName::ConditionalModalLogic => { "ConditionalModalLogic" }
            LogicName::ConditionalExtModalLogic => { "ConditionalExtModalLogic" }
            LogicName::IntuitionisticLogic => { "IntuitionisticLogic" }
            LogicName::MinimalFirstDegreeEntailmentLogic => { "MinimalFirstDegreeEntailmentLogic" }
        });
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
