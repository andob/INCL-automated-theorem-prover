use std::any::Any;
use std::rc::Rc;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules, ModalityRef};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::{ManyValuedContradictionBehaviour, ManyValuedLogicSemantics};

//check out book chapters 8 and 11a
pub struct KleeneModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl KleeneModalLogic
{
    pub fn K3_K() -> KleeneModalLogic { KleeneModalLogic { name:LogicName::of("Kleene+KModalLogic"), is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn K3_T() -> KleeneModalLogic { KleeneModalLogic { name:LogicName::of("Kleene+TModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn K3_B() -> KleeneModalLogic { KleeneModalLogic { name:LogicName::of("Kleene+BModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn K3_S4() -> KleeneModalLogic { KleeneModalLogic { name:LogicName::of("Kleene+S4ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn K3_S5() -> KleeneModalLogic { KleeneModalLogic { name:LogicName::of("Kleene+S5ModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for KleeneModalLogic
{
    fn get_name(&self) -> LogicName { self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ManyValuedLogicSemantics::new();
        semantics.add_behaviour(ManyValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus);
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

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl KleeneModalLogic
{
    pub fn get_modality(&self) -> Modality<KleeneModalLogic>
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
