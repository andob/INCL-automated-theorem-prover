use std::any::Any;
use str_macro::str;
use crate::logic::{Logic, LogicRule};
use crate::logic::common_modal_logic::{ModalLogicRules, Necessity, Possibility};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_semantics::BinarySemantics;
use crate::semantics::Semantics;

pub struct NormalModalLogic
{
    pub name : String,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl NormalModalLogic
{
    pub fn K() -> NormalModalLogic { NormalModalLogic { name: str!("KModalLogic"), is_reflexive: false, is_symmetric: false, is_transitive: false }}
    pub fn T() -> NormalModalLogic { NormalModalLogic { name: str!("TModalLogic"), is_reflexive: true, is_symmetric: false, is_transitive: false }}
    pub fn B() -> NormalModalLogic { NormalModalLogic { name: str!("BModalLogic"), is_reflexive: true, is_symmetric: true, is_transitive: false }}
    pub fn S4() -> NormalModalLogic { NormalModalLogic { name: str!("S4ModalLogic"), is_reflexive: true, is_symmetric: false, is_transitive: true }}
    pub fn S5() -> NormalModalLogic { NormalModalLogic { name: str!("S5ModalLogic"), is_reflexive: true, is_symmetric: true, is_transitive: true }}
}

impl Logic for NormalModalLogic
{
    fn get_name(&self) -> &str { self.name.as_str() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinarySemantics{});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply, TokenTypeID::StrictImply,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(ModalLogicRules::new(self.get_possibility(), self.get_necessity())),
        ];
    }
}

impl NormalModalLogic
{
    pub fn get_possibility(&self) -> Possibility<NormalModalLogic>
    {
        return Possibility
        {
            is_applicable: |_, _, _| { true },
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_reflexive { graph.add_missing_reflexive_vertices() }
                if logic.is_symmetric { graph.add_missing_symmetric_vertices() }
                if logic.is_transitive { graph.add_missing_transitive_vertices() }
            }
        };
    }

    pub fn get_necessity(&self) -> Necessity<NormalModalLogic>
    {
        //todo modify this
        return Necessity
        {
            is_applicable: |_, _| { true },
            dummy: |logic| {},
        };
    }
}
