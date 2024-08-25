use std::any::Any;
use std::rc::Rc;
use box_macro::bx;
use crate::formula::Formula::{Imply, Non, Or};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules, ModalityRef};
use crate::logic::first_degree_entailment::FirstDegreeEntailmentLogicRules;
use crate::logic::first_degree_entailment::generic_biimply_fde_rule::GenericBiImplyAsConjunctionRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::semantics::many_valued_logic_semantics::{ManyValuedContradictionBehaviour, ManyValuedLogicSemantics};
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 8 and 11a
pub struct RMingle3ModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl RMingle3ModalLogic
{
    pub fn RM3_K() -> RMingle3ModalLogic { RMingle3ModalLogic { name:LogicName::of("RMingle3+KModalLogic"), is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn RM3_T() -> RMingle3ModalLogic { RMingle3ModalLogic { name:LogicName::of("RMingle3+TModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn RM3_B() -> RMingle3ModalLogic { RMingle3ModalLogic { name:LogicName::of("RMingle3+BModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn RM3_S4() -> RMingle3ModalLogic { RMingle3ModalLogic { name:LogicName::of("RMingle3+S4ModalLogic"), is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn RM3_S5() -> RMingle3ModalLogic { RMingle3ModalLogic { name:LogicName::of("RMingle3+S5ModalLogic"), is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for RMingle3ModalLogic
{
    fn get_name(&self) -> LogicName { self.name.clone() }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        let mut semantics = ManyValuedLogicSemantics::new();
        semantics.add_behaviour(ManyValuedContradictionBehaviour::FormulaMinusWithNonFormulaMinus);
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
            Box::new(RMingle3ImplicationRules::new(modality)),
            Box::new(GenericBiImplyAsConjunctionRule {}),
        ]
    }

    fn get_modality_ref(&self) -> Option<ModalityRef>
    {
        return Some(ModalityRef::new(self.get_modality()));
    }
}

impl RMingle3ModalLogic
{
    pub fn get_modality(&self) -> Modality<RMingle3ModalLogic>
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

struct RMingle3ImplicationRules
{
    modality : Rc<Modality<RMingle3ModalLogic>>
}

impl RMingle3ImplicationRules
{
    fn new(modality : Rc<Modality<RMingle3ModalLogic>>) -> RMingle3ImplicationRules
    {
        return RMingle3ImplicationRules { modality };
    }
}

impl LogicRule for RMingle3ImplicationRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

        return match &node.formula
        {
            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let p_minus = p.with_sign(Minus);
                let p_minus_node = factory.new_node(p_minus);

                let non_q_minus = Non(bx!(q.clone()), extras.clone()).with_sign(Minus);
                let non_q_minus_node = factory.new_node(non_q_minus);

                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Plus);
                let q_or_non_q_minus = Or(bx!(q.clone()), bx!(non_q), extras.clone()).with_sign(Plus);
                let q_or_non_q_minus_node = factory.new_node(q_or_non_q_minus);

                let non_p = Non(bx!(p.clone()), extras.clone()).with_sign(Plus);
                let p_or_non_p_minus = Or(bx!(p.clone()), bx!(non_p), extras.clone()).with_sign(Plus);
                let p_or_non_p_minus_node = factory.new_node_with_subnode(p_or_non_p_minus, q_or_non_q_minus_node);

                return Some(ProofSubtree::with_left_middle_right_nodes(p_minus_node, non_q_minus_node, p_or_non_p_minus_node));
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

                let p_node = factory.new_node_with_subnode(p.clone(), non_q_node);

                return Some(ProofSubtree::with_middle_node(p_node));
            }

            Non(box Imply(box p, box q, _), extras) if extras.sign == Minus =>
            {
                let p_node = factory.new_node(p.clone());

                let non_q = Non(bx!(q.clone()), extras.clone()).with_sign(Minus);
                let non_q_node = factory.new_node(non_q);

                return Some(ProofSubtree::with_left_right_nodes(p_node, non_q_node));
            }

            _ => None
        }
    }
}
