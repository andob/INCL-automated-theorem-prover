use std::collections::BTreeSet;
use box_macro::bx;
use FirstOrderLogicDomainType::VariableDomain;
use crate::formula::Formula::{DefinitelyExists, Exists, ForAll, Non};
use crate::formula::{Formula, FormulaExtras, PossibleWorld, PredicateArgument};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::first_order_logic::exists_quantifier_rule::ExistsQuantifierRule;
use crate::logic::first_order_logic::{FirstOrderLogic, FirstOrderLogicDomainType};
use crate::logic::first_order_logic::FirstOrderLogicDomainType::ConstantDomain;
use crate::logic::first_order_logic::predicate_args_with_equivalences::create_equality_formulas_filtering_lambda;
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct ForAllQuantifierRule {}

impl LogicRule for ForAllQuantifierRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box ForAll(x, box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.with_is_hidden(false));
                let exists_non_p = Exists(x.clone(), bx!(non_p), extras.with_is_hidden(false));
                let exists_non_p_node = factory.new_node(exists_non_p);

                return Some(ProofSubtree::with_middle_node(exists_non_p_node));
            }

            ForAll(x, box p, extras) if extras.sign == Plus =>
            {
                return self.apply_for_all_quantification(factory, node, x, p, &extras.with_is_hidden(false));
            }

            Exists(x, box p, extras) if extras.sign == Minus =>
            {
                return self.apply_for_all_quantification(factory, node, x, p, &extras.with_is_hidden(false));
            }

            _ => None
        }
    }
}

struct ForAllQuantifierOutputNodes
{
    domain_type : FirstOrderLogicDomainType,
    nodes_on_left : Vec<ProofTreeNode>,
    nodes_on_right : Vec<ProofTreeNode>,
}

impl ForAllQuantifierOutputNodes
{
    pub fn new(domain_type : FirstOrderLogicDomainType) -> ForAllQuantifierOutputNodes
    {
        return ForAllQuantifierOutputNodes { domain_type, nodes_on_left:vec![], nodes_on_right:vec![] };
    }

    pub fn add(&mut self, factory : &mut RuleApplyFactory, binded_x : PredicateArgument, binded_p : Formula, extras : &FormulaExtras)
    {
        let binded_p_node = factory.new_node(binded_p);
        self.nodes_on_right.push(binded_p_node);

        if matches!(self.domain_type, VariableDomain(..))
        {
            let definitely_exists_x = DefinitelyExists(binded_x, extras.clone());
            if factory.get_logic().get_semantics().number_of_truth_values() == 2
            {
                let non_definitely_exists_x = Non(bx!(definitely_exists_x), extras.clone());
                let non_definitely_exists_x_node = factory.new_node(non_definitely_exists_x);
                self.nodes_on_left.push(non_definitely_exists_x_node);
            }
            else
            {
                let minus_definitely_exists_x = definitely_exists_x.with_sign(Minus);
                let minus_definitely_exists_x_node = factory.new_node(minus_definitely_exists_x);
                self.nodes_on_left.push(minus_definitely_exists_x_node);
            }
        }
    }

    pub fn is_empty(&self) -> bool
    {
        return self.nodes_on_right.is_empty();
    }

    pub fn to_proof_subtree(self, factory : &mut RuleApplyFactory) -> ProofSubtree
    {
        if matches!(self.domain_type, VariableDomain(..))
        {
            let zipped_nodes = self.nodes_on_left
                .into_iter().zip(self.nodes_on_right.into_iter())
                .collect::<Vec<(ProofTreeNode, ProofTreeNode)>>();

            return ProofSubtree::with_zipped_left_right_vertical_nodes(zipped_nodes, factory.tree_node_factory);
        }

        return ProofSubtree::with_middle_vertical_nodes(self.nodes_on_right);
    }
}

impl ForAllQuantifierRule
{
    fn apply_for_all_quantification(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<FirstOrderLogic>()?;

        let mut output_nodes = ForAllQuantifierOutputNodes::new(logic.domain_type);
        self.apply_for_all_quantification_impl(factory, node, x, p, extras, &mut output_nodes);

        if output_nodes.is_empty()
        {
            //there are no objects to be iterated, act similar to exists quantifier
            let object_name_factory = ExistsQuantifierRule{}.get_object_name_factory(factory, node);
            let (instantiated_p, instantiated_x) = p.instantiated(x, &object_name_factory, extras);
            output_nodes.add(factory, instantiated_x.unwrap_or(x.clone()), instantiated_p, extras);
        }

        return Some(output_nodes.to_proof_subtree(factory));
    }

    fn apply_for_all_quantification_impl(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
        output_nodes : &mut ForAllQuantifierOutputNodes,
    )
    {
        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<FirstOrderLogic>().unwrap();

        let all_formulas_on_path = factory.tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .collect::<Vec<Formula>>();

        let all_args_on_path = all_formulas_on_path.iter()
            .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
            .collect::<BTreeSet<PredicateArgument>>();

        let args_that_definitely_exists = get_args_that_definitely_exists(&all_formulas_on_path, extras.possible_world);
        let variable_domain_check = |a : &&PredicateArgument| args_that_definitely_exists.iter().any(|d| d==a);

        let free_args = all_args_on_path.iter()
            .filter(|y| y.object_name == y.variable_name && !all_formulas_on_path.iter()
                .any(|formula| formula.contains_quantifier_with_argument(y)))
            .collect::<BTreeSet<&PredicateArgument>>();

        let instantiated_xs = all_args_on_path.iter()
            .filter(|a| a.is_instantiated() && a.variable_name == x.variable_name && a.is_rigid_designator)
            .filter(|a| logic.domain_type == ConstantDomain || variable_domain_check(a))
            .collect::<BTreeSet<&PredicateArgument>>();
        for instantiated_x in instantiated_xs
        {
            let (binded_p, binded_x) = p.binded(x, instantiated_x.object_name.clone(), extras);
            output_nodes.add(factory, binded_x.unwrap_or(x.clone()), binded_p, extras);
        }

        let equivalences = all_formulas_on_path.iter()
            .filter_map(create_equality_formulas_filtering_lambda())
            .collect::<BTreeSet<(&PredicateArgument, &PredicateArgument)>>();

        let has_no_equivalent = |x : &PredicateArgument| !equivalences.iter()
            .any(|(y, z)| x.variable_name == *y.variable_name || x.variable_name == *z.variable_name);

        if has_no_equivalent(x)
        {
            let ys_with_no_equivalent = all_args_on_path.iter()
                .filter(|y| x.variable_name != y.variable_name)
                .filter(|y| (!y.is_instantiated() && has_no_equivalent(y)) || free_args.contains(y))
                .collect::<BTreeSet<&PredicateArgument>>();

            for y in ys_with_no_equivalent
            {
                let instantiated_ys = all_args_on_path.iter()
                    .filter(|a| a.variable_name == y.variable_name && a.is_rigid_designator)
                    .filter(|a| free_args.contains(a) || a.is_instantiated())
                    .filter(|a| logic.domain_type == ConstantDomain || variable_domain_check(a))
                    .collect::<BTreeSet<&PredicateArgument>>();
                for instantiated_y in instantiated_ys
                {
                    let (binded_p, binded_x) = p.binded(x, instantiated_y.object_name.clone(), extras);
                    output_nodes.add(factory, binded_x.unwrap_or(x.clone()), binded_p, extras);
                }
            }
        }
        else
        {
            let equivalent_ys = equivalences.iter()
                .filter(|(y, z)| x.variable_name == *y.variable_name || x.variable_name == *z.variable_name)
                .map(|(y, z)| if x.variable_name == *y.variable_name { *z } else { *y })
                .collect::<BTreeSet<&PredicateArgument>>();

            for y in equivalent_ys
            {
                let instantiated_ys = all_args_on_path.iter()
                    .filter(|a| a.variable_name == y.variable_name && a.is_rigid_designator)
                    .filter(|a| free_args.contains(a) || a.is_instantiated())
                    .filter(|a| logic.domain_type == ConstantDomain || variable_domain_check(a))
                    .collect::<BTreeSet<&PredicateArgument>>();
                for instantiated_y in instantiated_ys
                {
                    let (binded_p, binded_x) = p.binded(x, instantiated_y.object_name.clone(), extras);
                    output_nodes.add(factory, binded_x.unwrap_or(x.clone()), binded_p, extras);
                }
            }
        }
    }
}

pub fn get_args_that_definitely_exists(all_formulas_on_path : &Vec<Formula>, possible_world : PossibleWorld) -> BTreeSet<&PredicateArgument>
{
    let mut args_that_definitely_exists = all_formulas_on_path.iter()
        .filter(|formula| formula.get_possible_world() == possible_world && formula.get_sign() == Plus)
        .filter_map(|formula| if let DefinitelyExists(x, _) = formula { Some(x) } else { None })
        .collect::<BTreeSet<&PredicateArgument>>();

    let mut equivalences_of_args_that_definitely_exists = all_formulas_on_path.iter()
        .filter(|formula| formula.get_possible_world() == possible_world)
        .filter_map(create_equality_formulas_filtering_lambda())
        .filter(|(x, y)| args_that_definitely_exists.iter().any(|d| x==d || y==d))
        .flat_map(|(x, y)| vec![x, y])
        .collect::<BTreeSet<&PredicateArgument>>();

    args_that_definitely_exists.append(&mut equivalences_of_args_that_definitely_exists);
    return args_that_definitely_exists;
}
