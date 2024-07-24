use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use strum::IntoEnumIterator;
use crate::formula::Formula::{BiImply, Imply, Non, Or, And};
use crate::formula::FormulaExtras;
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{BaseLogicNameIndex, Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::first_degree_entailment::generic_biimply_fde_rule::GenericBiImplyAsConjunctionRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::three_valued_logic_semantics::{ThreeValuedContradictionBehaviour, ThreeValuedLogicSemantics};
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 8 and 11a
pub struct LukasiewiczModalLogic
{
    pub base_name: LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl LukasiewiczModalLogic
{
    pub fn L3_K() -> LukasiewiczModalLogic { LukasiewiczModalLogic { base_name:LogicName::KModalLogic, is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn L3_T() -> LukasiewiczModalLogic { LukasiewiczModalLogic { base_name:LogicName::TModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn L3_B() -> LukasiewiczModalLogic { LukasiewiczModalLogic { base_name:LogicName::BModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn L3_S4() -> LukasiewiczModalLogic { LukasiewiczModalLogic { base_name:LogicName::S4ModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn L3_S5() -> LukasiewiczModalLogic { LukasiewiczModalLogic { base_name:LogicName::S5ModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for LukasiewiczModalLogic
{
    fn get_name(&self) -> LogicName
    {
        let base_name_index = LogicName::iter().position(|name| self.base_name==name).unwrap();
        return LogicName::LukasiewiczModalLogic(base_name_index as BaseLogicNameIndex);
    }

    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ThreeValuedLogicSemantics::new();
        semantics.add_behaviour(ThreeValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus);
        return Box::new(semantics);
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        let modality = Rc::new(self.get_modality());
        return vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {}),
            Box::new(ModalLogicRules::new(modality.clone())),
            Box::new(LukasiewiczImplicationRules::new(modality)),
            Box::new(GenericBiImplyAsConjunctionRule {}),
        ]
    }
}

impl LukasiewiczModalLogic
{
    pub fn get_modality(&self) -> Modality<LukasiewiczModalLogic>
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

struct LukasiewiczImplicationRules
{
    modality : Rc<Modality<LukasiewiczModalLogic>>
}

impl LukasiewiczImplicationRules
{
    fn new(modality : Rc<Modality<LukasiewiczModalLogic>>) -> LukasiewiczImplicationRules
    {
        return LukasiewiczImplicationRules { modality };
    }
}

impl LogicRule for LukasiewiczImplicationRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

        return match &node.formula
        {
            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let p = p.in_world(extras.possible_world);
                let non_p_plus = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let non_p_plus_node = factory.new_node(non_p_plus);

                let q = q.in_world(extras.possible_world);
                let q_plus = q.with_sign(Plus);
                let q_plus_node = factory.new_node(q_plus);

                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let q_or_non_q_minus = Or(bx!(q), bx!(non_q), extras.clone()).with_sign(Minus);
                let q_or_non_q_minus_node = factory.new_node(q_or_non_q_minus);

                let non_p = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let p_or_non_p_minus = Or(bx!(p), bx!(non_p), extras.clone()).with_sign(Minus);
                let p_or_non_p_minus_node = factory.new_node_with_subnode(p_or_non_p_minus, q_or_non_q_minus_node);

                return Some(ProofSubtree::with_left_middle_right_nodes(non_p_plus_node, q_plus_node, p_or_non_p_minus_node));
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let q_minus = q.with_sign(Minus);
                let q_minus_node = factory.new_node(q_minus);

                let p_plus = p.with_sign(Plus);
                let p_plus_node = factory.new_node_with_subnode(p_plus, q_minus_node);

                let non_p_minus = Non(bx!(p), extras.clone()).with_sign(Minus);
                let non_p_minus_node = factory.new_node(non_p_minus);

                let non_q_plus = Non(bx!(q), extras.clone()).with_sign(Plus);
                let non_q_plus_node = factory.new_node_with_subnode(non_q_plus, non_p_minus_node);

                return Some(ProofSubtree::with_left_right_nodes(p_plus_node, non_q_plus_node));
            }

            Non(box Imply(box p, box q, _), extras) if extras.sign == Plus =>
            {
                let q = q.in_world(extras.possible_world);
                let non_q = Non(bx!(q), extras.clone()).with_sign(Plus);
                let non_q_node = factory.new_node(non_q);

                let p = p.in_world(extras.possible_world).with_sign(Plus);
                let p_node = factory.new_node_with_subnode(p, non_q_node);

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            Non(box Imply(box p, box q, _), extras) if extras.sign == Minus =>
            {
                let p = p.in_world(extras.possible_world).with_sign(Minus);
                let p_node = factory.new_node(p);

                let q = q.in_world(extras.possible_world);
                let non_q = Non(bx!(q), extras.clone()).with_sign(Minus);
                let non_q_node = factory.new_node(non_q);

                return Some(ProofSubtree::with_left_right_nodes(p_node, non_q_node));
            }

            BiImply(box p, box q, extras) =>
            {
                let p = p.in_world(extras.possible_world);
                let q = q.in_world(extras.possible_world);

                let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone()).with_sign(Plus);
                let q_imply_p = Imply(bx!(q), bx!(p), extras.clone()).with_sign(Plus);

                let conjunction = And(bx!(p_imply_q), bx!(q_imply_p), FormulaExtras
                {
                    possible_world: extras.possible_world,
                    is_hidden: true, sign: Plus
                });

                let conjunction_node = factory.new_node(conjunction);
                return Some(ProofSubtree::with_middle_node(conjunction_node));
            }

            _ => None
        }
    }
}
