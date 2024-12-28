use crate::formula::Formula::{Imply, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::common_modal_logic::{ModalLogicRules, Modality, ModalityRef};
use crate::logic::first_degree_entailment::generic_biimply_fde_rule::GenericBiImplyAsConjunctionRule;
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::parser::token_types::TokenTypeID;
use crate::semantics::many_valued_logic_semantics::{ManyValuedContradictionBehaviour, ManyValuedLogicSemantics};
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;
use box_macro::bx;
use std::any::Any;
use std::rc::Rc;

//check out book chapters 8 and 11a
pub struct LukasiewiczModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl LukasiewiczModalLogic
{
    pub fn L3_K() -> LukasiewiczModalLogic { LukasiewiczModalLogic { name:LogicName::of("Lukasiewicz+KModalLogic"), is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn L3_T() -> LukasiewiczModalLogic { LukasiewiczModalLogic { name:LogicName::of("Lukasiewicz+TModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn L3_B() -> LukasiewiczModalLogic { LukasiewiczModalLogic { name:LogicName::of("Lukasiewicz+BModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn L3_S4() -> LukasiewiczModalLogic { LukasiewiczModalLogic { name:LogicName::of("Lukasiewicz+S4ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn L3_S5() -> LukasiewiczModalLogic { LukasiewiczModalLogic { name:LogicName::of("Lukasiewicz+S5ModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for LukasiewiczModalLogic
{
    fn get_name(&self) -> LogicName { self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ManyValuedLogicSemantics::with_three_values();
        semantics.add_behaviour(ManyValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus);
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

    fn get_rules(&self) -> LogicRuleCollection
    {
        let modality = Rc::new(self.get_modality());
        return LogicRuleCollection::of(vec!
        [
            Box::new(FirstDegreeEntailmentLogicRules {}),
            Box::new(ModalLogicRules::new(modality.clone())),
            Box::new(LukasiewiczImplicationRules::new(modality)),
            Box::new(GenericBiImplyAsConjunctionRule {}),
        ])
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
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
        return match &node.formula
        {
            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let non_p_plus = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let non_p_plus_node = factory.new_node(non_p_plus);

                let q_plus = q.with_sign(Plus);
                let q_plus_node = factory.new_node(q_plus);

                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let q_or_non_q_minus = Or(bx!(q.clone()), bx!(non_q), extras.clone()).with_sign(Minus);
                let q_or_non_q_minus_node = factory.new_node(q_or_non_q_minus);

                let non_p = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let p_or_non_p_minus = Or(bx!(p.clone()), bx!(non_p), extras.clone()).with_sign(Minus);
                let p_or_non_p_minus_node = factory.new_node_with_subnode(p_or_non_p_minus, q_or_non_q_minus_node);

                return Some(ProofSubtree::with_left_middle_right_nodes(non_p_plus_node, q_plus_node, p_or_non_p_minus_node));
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let q_minus = q.with_sign(Minus);
                let q_minus_node = factory.new_node(q_minus);

                let p_plus = p.with_sign(Plus);
                let p_plus_node = factory.new_node_with_subnode(p_plus, q_minus_node);

                let non_p_minus = Non(bx!(p.clone()), extras.clone()).with_sign(Minus);
                let non_p_minus_node = factory.new_node(non_p_minus);

                let non_q_plus = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let non_q_plus_node = factory.new_node_with_subnode(non_q_plus, non_p_minus_node);

                return Some(ProofSubtree::with_left_right_nodes(p_plus_node, non_q_plus_node));
            }

            Non(box Imply(box p, box q, _), extras) if extras.sign == Plus =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let non_q_node = factory.new_node(non_q);

                let plus_p = p.with_sign(Plus);
                let plus_p_node = factory.new_node_with_subnode(plus_p, non_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            Non(box Imply(box p, box q, _), extras) if extras.sign == Minus =>
            {
                let minus_p = p.with_sign(Minus);
                let minus_p_node = factory.new_node(minus_p);

                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Minus);
                let non_q_node = factory.new_node(non_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, non_q_node));
            }

            _ => None
        }
    }
}
