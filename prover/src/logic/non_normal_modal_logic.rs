use std::any::Any;
use std::rc::Rc;
use crate::formula::PossibleWorld;
use crate::formula::Formula::{Necessary, StrictImply};
use crate::logic::{Logic, LogicName, LogicRuleCollection};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;

//check out book chapter 4
pub struct NonNormalModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl NonNormalModalLogic
{
    pub fn N() -> NonNormalModalLogic { NonNormalModalLogic { name:LogicName::of("NModalLogic"), is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn S2() -> NonNormalModalLogic { NonNormalModalLogic { name:LogicName::of("S2ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn S3() -> NonNormalModalLogic { NonNormalModalLogic { name:LogicName::of("S3ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn S3_5() -> NonNormalModalLogic { NonNormalModalLogic { name:LogicName::of("S3.5ModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for NonNormalModalLogic
{
    fn get_name(&self) -> LogicName { self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinaryLogicSemantics {});
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
        ]
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(ModalLogicRules::new(Rc::new(self.get_modality()))),
        ])
    }
}

impl NonNormalModalLogic
{
    pub fn get_modality(&self) -> Modality<NonNormalModalLogic>
    {
        return Modality
        {
            is_possibility_applicable: |factory, node, extras|
            {
                extras.possible_world == PossibleWorld::zero() ||
                factory.tree.get_path_that_goes_through_node(node).nodes.iter().any(|node|
                    node.formula.get_possible_world() == extras.possible_world &&
                    matches!(node.formula, Necessary(..) | StrictImply(..)))
            },
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
