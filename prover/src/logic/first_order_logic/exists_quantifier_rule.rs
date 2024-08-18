use std::collections::BTreeSet;
use box_macro::bx;
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, Equals, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PredicateArgument, PredicateArguments};
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct ExistsQuantifierRule {}

impl LogicRule for ExistsQuantifierRule
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        if let Non(box Exists(x, box p, _), extras) = &node.formula
        {
            let non_p = Non(bx!(p.clone()), extras.clone());
            let for_all_non_p = ForAll(x.clone(), bx!(non_p), extras.clone());
            let for_all_non_p_node = factory.new_node(for_all_non_p);

            return Some(ProofSubtree::with_middle_node(for_all_non_p_node));
        }

        if let Exists(x, box p, extras) = &node.formula
        {
            let object_name_factory = Self::get_object_name_factory(factory, node);
            let instantiated_p = p.instantiated(x, &object_name_factory, extras);
            let instantiated_p_node = factory.new_node(instantiated_p);

            return Some(ProofSubtree::with_middle_node(instantiated_p_node));
        }

        return None;
    }
}

impl ExistsQuantifierRule
{
    pub fn get_object_name_factory(factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Box<dyn Fn() -> String>
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
    pub fn binded(&self, x : &PredicateArgument, binding_name : String, extras : &FormulaExtras) -> Formula
    {
        let object_name_factory : Box<dyn Fn() -> String> = Box::new(move || binding_name.clone());
        return self.instantiated(x, &object_name_factory, extras);
    }

    pub fn instantiated(&self, x : &PredicateArgument, object_name_factory : &Box<dyn Fn() -> String>, extras : &FormulaExtras) -> Formula
    {
        let mut instantiated_box = |p : &Box<Formula>| Box::new(p.instantiated(x, &object_name_factory, extras));

        return match self
        {
            Atomic(p, old_extras) =>
            {
                let new_extras = AtomicFormulaExtras
                {
                    predicate_args: old_extras.predicate_args.instantiated(x, &object_name_factory),
                    possible_world: extras.possible_world,
                    is_hidden: old_extras.is_hidden,
                    sign: old_extras.sign,
                };

                return Atomic(p.clone(), new_extras);
            }

            Equals(y, z, _) =>
            {
                if !y.is_instantiated() && y.variable_name == x.variable_name
                {
                    let mut instantiated_y = y.clone();
                    instantiated_y.object_name = (*object_name_factory)();
                    return Equals(instantiated_y, z.clone(), extras.clone());
                }

                if !z.is_instantiated() && z.variable_name == x.variable_name
                {
                    let mut instantiated_z = z.clone();
                    instantiated_z.object_name = (*object_name_factory)();
                    return Equals(y.clone(), instantiated_z, extras.clone());
                }

                return Equals(x.clone(), y.clone(), extras.clone());
            }

            Non(p, _) => { Non(instantiated_box(p), extras.clone()) }
            And(p, q, _) => { And(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Or(p, q, _) => { Or(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Imply(p, q, _) => { Imply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            BiImply(p, q, _) => { BiImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            StrictImply(p, q, _) => { StrictImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Conditional(p, q, _) => { Conditional(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Exists(x, p, _) => { Exists(x.clone(), instantiated_box(p), extras.clone()) }
            ForAll(x, p, _) => { ForAll(x.clone(), instantiated_box(p), extras.clone()) }
            Possible(p, _) => { Possible(instantiated_box(p), extras.clone()) }
            Necessary(p, _) => { Necessary(instantiated_box(p), extras.clone()) }
            InPast(p, _) => { InPast(instantiated_box(p), extras.clone()) }
            InFuture(p, _) => { InFuture(instantiated_box(p), extras.clone()) }
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