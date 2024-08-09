use std::any::Any;
use std::collections::BTreeSet;
use std::rc::Rc;
use itertools::Itertools;
use crate::formula::{Formula, PredicateArgument};
use crate::graph::GraphVertex;
use crate::logic::{Logic, LogicName, LogicRule};
use crate::logic::common_modal_logic::{Modality, ModalLogicRules};
use crate::logic::first_order_logic::QuantifierRules;
use crate::logic::propositional_logic::PropositionalLogicRules;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::binary_logic_semantics::BinaryLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

//check out book chapters 14 and 16
pub struct FirstOrderModalLogic
{
    pub name : LogicName,
    pub is_reflexive : bool,
    pub is_symmetric : bool,
    pub is_transitive : bool,
}

#[allow(non_snake_case)]
impl FirstOrderModalLogic
{
    pub fn CK() -> FirstOrderModalLogic { FirstOrderModalLogic { name:LogicName::CKModalLogic, is_reflexive:false, is_symmetric:false, is_transitive:false }}
    pub fn CT() -> FirstOrderModalLogic { FirstOrderModalLogic { name:LogicName::CTModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:false }}
    pub fn CB() -> FirstOrderModalLogic { FirstOrderModalLogic { name:LogicName::CBModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:false }}
    pub fn CS4() -> FirstOrderModalLogic { FirstOrderModalLogic { name:LogicName::CS4ModalLogic, is_reflexive:true, is_symmetric:false, is_transitive:true }}
    pub fn CS5() -> FirstOrderModalLogic { FirstOrderModalLogic { name:LogicName::CS5ModalLogic, is_reflexive:true, is_symmetric:true, is_transitive:true }}
}

impl Logic for FirstOrderModalLogic
{
    fn get_name(&self) -> LogicName { self.name }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(BinaryLogicSemantics {});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithArgs,
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Exists, TokenTypeID::ForAll, TokenTypeID::Equals,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply, TokenTypeID::StrictImply,
            TokenTypeID::Necessary, TokenTypeID::Possible,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ]
    }

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>
    {
        let modality = Rc::new(self.get_modality());
        return vec!
        [
            Box::new(PropositionalLogicRules {}),
            Box::new(QuantifierRules {}),
            Box::new(IdentityInvarianceRule::new(modality.clone())),
            Box::new(ModalLogicRules::new(modality)),
        ]
    }
}

impl FirstOrderModalLogic
{
    pub fn get_modality(&self) -> Modality<FirstOrderModalLogic>
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

struct IdentityInvarianceRule
{
    modality : Rc<Modality<FirstOrderModalLogic>>
}

impl IdentityInvarianceRule
{
    pub fn new(modality : Rc<Modality<FirstOrderModalLogic>>) -> IdentityInvarianceRule
    {
        return IdentityInvarianceRule { modality };
    }
}

impl LogicRule for IdentityInvarianceRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        self.modality.initialize_graph_if_needed(factory);

        if let Formula::Possible(box p, extras) = &node.formula
        {
            let original_world = extras.possible_world;
            let original_graph_vertices = factory.modality_graph.vertices.clone();

            let equalities_in_original_world = factory
                .tree.get_paths_that_goes_through_node(node).into_iter()
                .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
                .filter(|formula| formula.get_possible_world() == original_world)
                .filter_map(|formula| if let Formula::Equals(x, y, _) = formula { Some((x,y)) } else { None })
                .collect::<BTreeSet<(PredicateArgument, PredicateArgument)>>();

            if let Some(mut subtree) = self.modality.apply_possibility(factory, node, p, extras)
            {
                if equalities_in_original_world.is_empty() { return Some(subtree) };

                let new_graph_vertices = factory.modality_graph.vertices.iter()
                    .filter(|vertex| !original_graph_vertices.contains(vertex))
                    .collect::<BTreeSet<&GraphVertex>>();

                let new_equality_formulas = new_graph_vertices.iter().map(|vertex| vertex.to)
                    .flat_map(|new_world| equalities_in_original_world.iter().map(move |(x, y)|
                        Formula::Equals(x.clone(), y.clone(), extras.in_world(new_world))))
                    .collect::<Vec<Formula>>();

                let new_equality_nodes = new_equality_formulas.into_iter()
                    .map(|formula| factory.new_node(formula))
                    .collect::<Vec<ProofTreeNode>>();

                subtree.append(ProofSubtree::with_middle_vertical_nodes(new_equality_nodes));
                return Some(subtree);
            }
        }

        return None;
    }
}
