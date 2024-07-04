use std::any::Any;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_semantics::BinarySemantics;
use crate::semantics::Semantics;

pub struct NormalModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl NormalModalLogic
{
    pub fn K() -> NormalModalLogic { NormalModalLogic { name:LogicName::KModalLogic, is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn T() -> NormalModalLogic { NormalModalLogic { name:LogicName::TModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn B() -> NormalModalLogic { NormalModalLogic { name:LogicName::BModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn S4() -> NormalModalLogic { NormalModalLogic { name:LogicName::S4ModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn S5() -> NormalModalLogic { NormalModalLogic { name:LogicName::S5ModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for NormalModalLogic
{
    fn get_name(&self) -> LogicName { self.name }
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
            Box::new(ModalLogicRules::new(self.get_modality())),
        ];
    }
}

impl NormalModalLogic
{
    pub fn get_modality(&self) -> Modality<NormalModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| { true },
            is_necessity_applicable: |_, _, _| { true },
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_reflexive { graph.add_missing_reflexive_vertices() }
                if logic.is_symmetric { graph.add_missing_symmetric_vertices() }
                if logic.is_transitive { graph.add_missing_transitive_vertices() }
            }
        };
    }
}
