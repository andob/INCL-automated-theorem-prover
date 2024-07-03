use std::any::Any;
use std::fmt::Display;
use std::rc::Rc;
use anyhow::{Context, Result};
use crate::logic::first_order_logic::FirstOrderLogic;
use crate::logic::non_normal_modal_logic::NonNormalModalLogic;
use crate::logic::normal_modal_logic::NormalModalLogic;
use crate::logic::propositional_logic::PropositionalLogic;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub mod propositional_logic;
pub mod first_order_logic;
mod normal_modal_logic;
mod non_normal_modal_logic;
mod common_modal_logic;
pub mod rule_apply_factory;

pub trait LogicRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>;
}

pub trait Logic : Any
{
    //the logic name, eg: PropositionalLogic
    fn get_name(&self) -> &str;

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
            .find(|logic| logic.get_name() == name.as_str())
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
        ];
    }
}
