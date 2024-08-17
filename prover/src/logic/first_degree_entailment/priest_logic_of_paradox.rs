use std::any::Any;
use std::rc::Rc;
use str_macro::str;
use strum::IntoEnumIterator;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::first_degree_entailment::lukasiewicz_modal_logic::LukasiewiczModalLogic;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::three_valued_logic_semantics::{ThreeValuedContradictionBehaviour, ThreeValuedLogicSemantics};

//check out book chapters 8 and 11a
pub struct PriestLPModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl PriestLPModalLogic
{
    pub fn LP_K() -> PriestLPModalLogic { PriestLPModalLogic { name:LogicName::of("LP+KModalLogic"), is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn LP_T() -> PriestLPModalLogic { PriestLPModalLogic { name:LogicName::of("LP+TModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn LP_B() -> PriestLPModalLogic { PriestLPModalLogic { name:LogicName::of("LP+BModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn LP_S4() -> PriestLPModalLogic { PriestLPModalLogic { name:LogicName::of("LP+S4ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn LP_S5() -> PriestLPModalLogic { PriestLPModalLogic { name:LogicName::of("LP+S5ModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for PriestLPModalLogic
{
    fn get_name(&self) -> LogicName { self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ThreeValuedLogicSemantics::new();
        semantics.add_behaviour(ThreeValuedContradictionBehaviour::FormulaMinusWithNonFormulaMinus);
        return Box::new(semantics);
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        return vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {}),
            Box::new(ModalLogicRules::new(Rc::new(self.get_modality()))),
        ]
    }
}

impl PriestLPModalLogic
{
    pub fn get_modality(&self) -> Modality<PriestLPModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |_, _, _| true,
            is_necessity_applicable: |_, _, _| true,
            add_missing_graph_vertices: |logic, graph|
            {
                if logic.is_reflexive { graph.add_missing_reflexive_vertices() }
                if logic.is_symmetric { graph.add_missing_symmetric_vertices() }
                if logic.is_transitive { graph.add_missing_transitive_vertices() }
            }
        }
    }
}
