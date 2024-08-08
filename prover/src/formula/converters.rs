use std::collections::BTreeSet;
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArgument, PredicateArguments, Sign};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, Equals, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};

mod extras_in_world;
mod extras_with_sign;
mod extras_with_is_hidden;

impl Formula
{
    //todo refactor this to act similar to instantiated: attach world recursively
    pub fn in_world(&self, possible_world : PossibleWorld) -> Formula
    {
        return match self
        {
            Atomic(p, extras) => { Atomic(p.clone(), extras.in_world(possible_world)) }
            Non(p, extras) => { Non(p.clone(), extras.in_world(possible_world)) }
            And(p, q, extras) => { And(p.clone(), q.clone(), extras.in_world(possible_world)) }
            Or(p, q, extras) => { Or(p.clone(), q.clone(), extras.in_world(possible_world)) }
            Imply(p, q, extras) => { Imply(p.clone(), q.clone(), extras.in_world(possible_world)) }
            BiImply(p, q, extras) => { BiImply(p.clone(), q.clone(), extras.in_world(possible_world)) }
            StrictImply(p, q, extras) => { StrictImply(p.clone(), q.clone(), extras.in_world(possible_world)) }
            Conditional(p, q, extras) => { Conditional(p.clone(), q.clone(), extras.in_world(possible_world)) }
            Exists(x, p, extras) => { Exists(x.clone(), p.clone(), extras.in_world(possible_world)) }
            ForAll(x, p, extras) => { ForAll(x.clone(), p.clone(), extras.in_world(possible_world)) }
            Equals(x, y, extras) => { Equals(x.clone(), y.clone(), extras.in_world(possible_world)) }
            Possible(p, extras) => { Possible(p.clone(), extras.in_world(possible_world)) }
            Necessary(p, extras) => { Necessary(p.clone(), extras.in_world(possible_world)) }
            InPast(p, extras) => { InPast(p.clone(), extras.in_world(possible_world)) }
            InFuture(p, extras) => { InFuture(p.clone(), extras.in_world(possible_world)) }
            Comment(payload) => { Comment(payload.clone()) }
        }
    }

    pub fn get_possible_world(&self) -> PossibleWorld
    {
        return match self
        {
            Atomic(_, extras) => { extras.possible_world }
            Non(_, extras) => { extras.possible_world }
            And(_, _, extras) => { extras.possible_world }
            Or(_, _, extras) => { extras.possible_world }
            Imply(_, _, extras) => { extras.possible_world }
            BiImply(_, _, extras) => { extras.possible_world }
            StrictImply(_, _, extras) => { extras.possible_world }
            Conditional(_, _, extras) => { extras.possible_world }
            Exists(_, _, extras) => { extras.possible_world }
            ForAll(_, _, extras) => { extras.possible_world }
            Equals(_, _, extras) => { extras.possible_world }
            Possible(_, extras) => { extras.possible_world }
            Necessary(_, extras) => { extras.possible_world }
            InPast(_, extras) => { extras.possible_world }
            InFuture(_, extras) => { extras.possible_world }
            Comment(_) => { PossibleWorld::zero() }
        }
    }

    pub fn with_sign(&self, sign : Sign) -> Formula
    {
        return match self
        {
            Atomic(p, extras) => { Atomic(p.clone(), extras.with_sign(sign)) }
            Non(p, extras) => { Non(p.clone(), extras.with_sign(sign)) }
            And(p, q, extras) => { And(p.clone(), q.clone(), extras.with_sign(sign)) }
            Or(p, q, extras) => { Or(p.clone(), q.clone(), extras.with_sign(sign)) }
            Imply(p, q, extras) => { Imply(p.clone(), q.clone(), extras.with_sign(sign)) }
            BiImply(p, q, extras) => { BiImply(p.clone(), q.clone(), extras.with_sign(sign)) }
            StrictImply(p, q, extras) => { StrictImply(p.clone(), q.clone(), extras.with_sign(sign)) }
            Conditional(p, q, extras) => { Conditional(p.clone(), q.clone(), extras.with_sign(sign)) }
            Exists(x, p, extras) => { Exists(x.clone(), p.clone(), extras.with_sign(sign)) }
            ForAll(x, p, extras) => { ForAll(x.clone(), p.clone(), extras.with_sign(sign)) }
            Equals(x, y, extras) => { Equals(x.clone(), y.clone(), extras.with_sign(sign)) }
            Possible(p, extras) => { Possible(p.clone(), extras.with_sign(sign)) }
            Necessary(p, extras) => { Necessary(p.clone(), extras.with_sign(sign)) }
            InPast(p, extras) => { InPast(p.clone(), extras.with_sign(sign)) }
            InFuture(p, extras) => { InFuture(p.clone(), extras.with_sign(sign)) }
            Comment(payload) => { Comment(payload.clone()) }
        }
    }

    pub fn get_sign(&self) -> Sign
    {
        return match self
        {
            Atomic(_, extras) => { extras.sign }
            Non(_, extras) => { extras.sign }
            And(_, _, extras) => { extras.sign }
            Or(_, _, extras) => { extras.sign }
            Imply(_, _, extras) => { extras.sign }
            BiImply(_, _, extras) => { extras.sign }
            StrictImply(_, _, extras) => { extras.sign }
            Conditional(_, _, extras) => { extras.sign }
            Exists(_, _, extras) => { extras.sign }
            ForAll(_, _, extras) => { extras.sign }
            Equals(_, _, extras) => { extras.sign }
            Possible(_, extras) => { extras.sign }
            Necessary(_, extras) => { extras.sign }
            InPast(_, extras) => { extras.sign }
            InFuture(_, extras) => { extras.sign }
            Comment(_) => { Sign::Plus }
        }
    }

    pub fn with_is_hidden(&self, is_hidden : bool) -> Formula
    {
        return match self
        {
            Atomic(p, extras) => { Atomic(p.clone(), extras.with_is_hidden(is_hidden)) }
            Non(p, extras) => { Non(p.clone(), extras.with_is_hidden(is_hidden)) }
            And(p, q, extras) => { And(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            Or(p, q, extras) => { Or(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            Imply(p, q, extras) => { Imply(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            BiImply(p, q, extras) => { BiImply(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            StrictImply(p, q, extras) => { StrictImply(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            Conditional(p, q, extras) => { Conditional(p.clone(), q.clone(), extras.with_is_hidden(is_hidden)) }
            Exists(x, p, extras) => { Exists(x.clone(), p.clone(), extras.with_is_hidden(is_hidden)) }
            ForAll(x, p, extras) => { ForAll(x.clone(), p.clone(), extras.with_is_hidden(is_hidden)) }
            Equals(x, y, extras) => { Equals(x.clone(), y.clone(), extras.with_is_hidden(is_hidden)) }
            Possible(p, extras) => { Possible(p.clone(), extras.with_is_hidden(is_hidden)) }
            Necessary(p, extras) => { Necessary(p.clone(), extras.with_is_hidden(is_hidden)) }
            InPast(p, extras) => { InPast(p.clone(), extras.with_is_hidden(is_hidden)) }
            InFuture(p, extras) => { InFuture(p.clone(), extras.with_is_hidden(is_hidden)) }
            Comment(payload) => { Comment(payload.clone()) }
        }
    }

    pub fn is_hidden(&self) -> bool
    {
        return match self
        {
            Atomic(_, extras) => { extras.is_hidden }
            Non(_, extras) => { extras.is_hidden }
            And(_, _, extras) => { extras.is_hidden }
            Or(_, _, extras) => { extras.is_hidden }
            Imply(_, _, extras) => { extras.is_hidden }
            BiImply(_, _, extras) => { extras.is_hidden }
            StrictImply(_, _, extras) => { extras.is_hidden }
            Conditional(_, _, extras) => { extras.is_hidden }
            Exists(_, _, extras) => { extras.is_hidden }
            ForAll(_, _, extras) => { extras.is_hidden }
            Equals(_, _, extras) => { extras.is_hidden }
            Possible(_, extras) => { extras.is_hidden }
            Necessary(_, extras) => { extras.is_hidden }
            InPast(_, extras) => { extras.is_hidden }
            InFuture(_, extras) => { extras.is_hidden }
            Comment(_) => { false }
        }
    }

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

    pub fn get_predicate_arguments_of_atomic(&self) -> Option<PredicateArguments>
    {
        let mut get_predicate_arguments_of_atomic_from_tuple = |(p, q) : (&Formula, &Formula)|
            p.get_predicate_arguments_of_atomic().or_else(|| q.get_predicate_arguments_of_atomic());

        return match self
        {
            Atomic(_, extras) => { Some(extras.predicate_args.clone()) }
            Non(p, _) => { p.get_predicate_arguments_of_atomic() }
            And(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Or(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Imply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            BiImply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            StrictImply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Conditional(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Exists(_, p, _) => { p.get_predicate_arguments_of_atomic() }
            ForAll(_, p, _) => { p.get_predicate_arguments_of_atomic() }
            Equals(_, _, _) => { None }
            Possible(p, _) => { p.get_predicate_arguments_of_atomic() }
            Necessary(p, _) => { p.get_predicate_arguments_of_atomic() }
            InPast(p, _) => { p.get_predicate_arguments_of_atomic() }
            InFuture(p, _) => { p.get_predicate_arguments_of_atomic() }
            Comment(_) => { None }
        }
    }

    pub fn get_all_predicate_arguments(&self) -> BTreeSet<PredicateArgument>
    {
        let mut output : BTreeSet<PredicateArgument> = BTreeSet::new();
        self.get_all_predicate_arguments_recursively(&mut output);
        return output;
    }

    fn get_all_predicate_arguments_recursively(&self, output : &mut BTreeSet<PredicateArgument>)
    {
        let mut get_all_predicate_arguments_recursively_from_tuple = |(p, q) : (&Formula, &Formula)|
        {
            p.get_all_predicate_arguments_recursively(output);
            q.get_all_predicate_arguments_recursively(output);
        };

        match self
        {
            Atomic(_, extras) =>
            {
                for predicate_arg in extras.predicate_args.iter()
                {
                    if !output.contains(predicate_arg)
                    {
                        output.insert(predicate_arg.clone());
                    }
                }
            }

            Exists(x, p, _) =>
            {
                output.insert(x.clone());
                p.get_all_predicate_arguments_recursively(output);
            }
            ForAll(x, p, _) =>
            {
                output.insert(x.clone());
                p.get_all_predicate_arguments_recursively(output);
            }

            Non(box p, _) => { p.get_all_predicate_arguments_recursively(output); }
            And(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            Or(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            Imply(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            BiImply(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            StrictImply(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            Conditional(box p, box q, _) => { get_all_predicate_arguments_recursively_from_tuple((p, q)) }
            Possible(box p, _) => { p.get_all_predicate_arguments_recursively(output) }
            Necessary(box p, _) => { p.get_all_predicate_arguments_recursively(output) }
            InPast(box p, _) => { p.get_all_predicate_arguments_recursively(output) }
            InFuture(box p, _) => { p.get_all_predicate_arguments_recursively(output) }

            _ => {}
        }
    }

    pub fn contains_quantifier_with_argument(&self, y : &PredicateArgument) -> bool
    {
        return match self
        {
            Exists(x, box p, _) =>
            {
                return if x == y { true }
                else { p.contains_quantifier_with_argument(y) }
            }
            ForAll(x, p, _) =>
            {
                return if x == y { true }
                else { p.contains_quantifier_with_argument(y) }
            }

            Non(box p, _) => { p.contains_quantifier_with_argument(y) }
            Possible(box p, _) => { p.contains_quantifier_with_argument(y) }
            Necessary(box p, _) => { p.contains_quantifier_with_argument(y) }
            InPast(box p, _) => { p.contains_quantifier_with_argument(y) }
            InFuture(box p, _) => { p.contains_quantifier_with_argument(y) }

            And(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }
            Or(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }
            Imply(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }
            BiImply(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }
            StrictImply(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }
            Conditional(box p, box q, _) => { p.contains_quantifier_with_argument(y) || q.contains_quantifier_with_argument(y) }

            _ => { false }
        }
    }
}

impl AtomicFormulaExtras
{
    pub fn to_formula_extras(&self) -> FormulaExtras
    {
        return FormulaExtras
        {
            possible_world: self.possible_world,
            is_hidden: self.is_hidden,
            sign: self.sign,
        }
    }
}