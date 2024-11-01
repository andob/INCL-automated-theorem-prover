use std::collections::BTreeSet;
use box_macro::bx;
use itertools::Itertools;
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, DefinitelyExists, Equals, Exists, ForAll, GreaterOrEqualThan, Imply, InFuture, InPast, LessThan, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PredicateArgument, PredicateArguments};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::first_order_logic::{FirstOrderLogic, FirstOrderLogicDomainType};
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct ExistsQuantifierRule {}

impl LogicRule for ExistsQuantifierRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Exists(x, box p, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.with_is_hidden(false));
                let for_all_non_p = ForAll(x.clone(), bx!(non_p), extras.with_is_hidden(false));
                let for_all_non_p_node = factory.new_node(for_all_non_p);

                return Some(ProofSubtree::with_middle_node(for_all_non_p_node));
            }

            Exists(x, box p, extras) if extras.sign == Plus =>
            {
                return self.apply_exists_quantification(factory, node, x, p, &extras.with_is_hidden(false));
            }

            ForAll(x, box p, extras) if extras.sign == Minus =>
            {
                return self.apply_exists_quantification(factory, node, x, p, &extras.with_is_hidden(false));
            }

            _ => None
        }
    }
}

impl ExistsQuantifierRule
{
    fn apply_exists_quantification(&self,
        factory : &mut RuleApplyFactory, node : &ProofTreeNode,
        x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
    ) -> Option<ProofSubtree>
    {
        let mut output_nodes: Vec<ProofTreeNode> = vec![];

        let object_name_factory = self.get_object_name_factory(factory, node);
        let (instantiated_p, instantiated_x) = p.instantiated(x, &object_name_factory, extras);
        let instantiated_p_node = factory.new_node(instantiated_p);
        output_nodes.push(instantiated_p_node);

        let logic_pointer = factory.get_logic().clone();
        let logic = logic_pointer.cast_to::<FirstOrderLogic>()?;
        if logic.domain_type == FirstOrderLogicDomainType::VariableDomain && instantiated_x.is_some()
        {
            let definitely_exists_x = DefinitelyExists(instantiated_x?, extras.with_sign(Plus));
            let definitely_exists_x_node = factory.new_node(definitely_exists_x);
            output_nodes.push(definitely_exists_x_node);
        }

        return Some(ProofSubtree::with_middle_vertical_nodes(output_nodes));
    }

    pub fn get_object_name_factory(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Box<dyn Fn() -> String>
    {
        let used_names = factory.tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
            .flat_map(|name| vec![name.object_name, name.variable_name])
            .collect::<BTreeSet<String>>();

        return Box::new(move ||
        {
            let mut char = 'a';
            let mut aux = 0u64;
            loop
            {
                let name = if aux==0 { char.to_string() }
                else { format!("{}{}", char, aux) };

                if !used_names.contains(&name) { return name; }

                if char < 'z' { char = ((char as u8) + 1) as char; }
                else { char = 'a'; aux += 1; }
            }
        });
    }
}

impl Formula
{
    pub fn binded(&self, x : &PredicateArgument, binding_name : String, extras : &FormulaExtras) -> (Formula, Option<PredicateArgument>)
    {
        let object_name_factory : Box<dyn Fn() -> String> = Box::new(move || binding_name.clone());

        return self.instantiated(x, &object_name_factory, extras);
    }

    pub fn instantiated(&self, x : &PredicateArgument, object_name_factory : &Box<dyn Fn() -> String>, extras : &FormulaExtras) -> (Formula, Option<PredicateArgument>)
    {
        let instantiated_p = self.instantiate_impl(x, object_name_factory).with_sign(extras.sign);
        let instantiated_x = instantiated_p.get_all_predicate_arguments().into_iter()
            .find(|y| y.variable_name == x.variable_name && y.is_instantiated());

        return (instantiated_p, instantiated_x);
    }

    fn instantiate_impl(&self, x : &PredicateArgument, object_name_factory : &Box<dyn Fn() -> String>) -> Formula
    {
        let mut instantiated_box = |p : &Box<Formula>| Box::new(p.instantiate_impl(x, &object_name_factory));

        return match self
        {
            Atomic(p, extras) =>
            {
                return Atomic(p.clone(), AtomicFormulaExtras
                {
                    predicate_args: extras.predicate_args.instantiated(x, &object_name_factory),
                    possible_world: extras.possible_world, sign: extras.sign,
                    fuzzy_tags: extras.fuzzy_tags.clone(), is_hidden: extras.is_hidden,
                });
            }

            Equals(y, z, extras) =>
            {
                if !y.is_instantiated() && x.variable_name == y.variable_name && y.variable_name == z.variable_name
                {
                    let mut instantiated_y = y.clone();
                    let mut instantiated_z = z.clone();
                    let object_name = (*object_name_factory)();
                    instantiated_y.object_name = object_name.clone();
                    instantiated_z.object_name = object_name;
                    return Equals(instantiated_y, instantiated_z, extras.clone());
                }
                else if !y.is_instantiated() && y.variable_name == x.variable_name
                {
                    let mut instantiated_y = y.clone();
                    instantiated_y.object_name = (*object_name_factory)();
                    return Equals(instantiated_y, z.clone(), extras.clone());
                }
                else if !z.is_instantiated() && z.variable_name == x.variable_name
                {
                    let mut instantiated_z = z.clone();
                    instantiated_z.object_name = (*object_name_factory)();
                    return Equals(y.clone(), instantiated_z, extras.clone());
                }

                return Equals(y.clone(), z.clone(), extras.clone());
            }

            DefinitelyExists(y, extras) =>
            {
                if !y.is_instantiated() && y.variable_name == x.variable_name
                {
                    let mut instantiated_y = y.clone();
                    instantiated_y.object_name = (*object_name_factory)();
                    return DefinitelyExists(instantiated_y, extras.clone());
                }

                return DefinitelyExists(y.clone(), extras.clone());
            }

            Non(p, extras) => { Non(instantiated_box(p), extras.clone()) }
            And(p, q, extras) => { And(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Or(p, q, extras) => { Or(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Imply(p, q, extras) => { Imply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            BiImply(p, q, extras) => { BiImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            StrictImply(p, q, extras) => { StrictImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Conditional(p, q, extras) => { Conditional(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Exists(x, p, extras) => { Exists(x.clone(), instantiated_box(p), extras.clone()) }
            ForAll(x, p, extras) => { ForAll(x.clone(), instantiated_box(p), extras.clone()) }
            Possible(p, extras) => { Possible(instantiated_box(p), extras.clone()) }
            Necessary(p, extras) => { Necessary(instantiated_box(p), extras.clone()) }
            InPast(p, extras) => { InPast(instantiated_box(p), extras.clone()) }
            InFuture(p, extras) => { InFuture(instantiated_box(p), extras.clone()) }
            LessThan(x, y, extras) => { LessThan(x.clone(), y.clone(), extras.clone()) }
            GreaterOrEqualThan(x, y, extras) => { GreaterOrEqualThan(x.clone(), y.clone(), extras.clone()) }
            Comment(payload) => { Comment(payload.clone()) }
        }
    }
}

impl PredicateArguments
{
    pub fn instantiated(&self, x : &PredicateArgument, object_name_factory : &Box<dyn Fn() -> String>) -> PredicateArguments
    {
        let mut instantiated_args : Vec<PredicateArgument> = vec![];
        for arg in self.iter()
        {
            if arg.variable_name != x.variable_name
            {
                //we need to instantiate x and this is y. skip.
                instantiated_args.push(arg.clone());
            }
            else if arg.is_instantiated()
            {
                //we need to instantiate x and this is already instantiated (a:x). keep the instantiation (a:x).
                instantiated_args.push(arg.clone());
            }
            else
            {
                //instantiate x into a:x
                let mut instantiated_arg = arg.clone();
                instantiated_arg.object_name = (*object_name_factory)();
                instantiated_args.push(instantiated_arg);
            }
        }

        return PredicateArguments::new(instantiated_args);
    }
}
