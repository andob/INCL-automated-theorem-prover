use std::collections::BTreeSet;
use box_macro::bx;
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArgument, PredicateArguments, Sign};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, DefinitelyExists, Equals, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};

mod extras_in_world;
mod extras_with_sign;
mod extras_with_is_hidden;

impl Formula
{
    pub fn in_world(&self, world : PossibleWorld) -> Formula
    {
        return match self
        {
            Atomic(p, extras) => { Atomic(p.clone(), extras.in_world(world)) }
            Non(box p, extras) => { Non(bx!(p.in_world(world)), extras.in_world(world)) }
            And(box p, box q, extras) => { And(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            Or(box p, box q, extras) => { Or(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            Imply(box p, box q, extras) => { Imply(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            BiImply(box p, box q, extras) => { BiImply(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            StrictImply(box p, box q, extras) => { StrictImply(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            Conditional(box p, box q, extras) => { Conditional(bx!(p.in_world(world)), bx!(q.in_world(world)), extras.in_world(world)) }
            Exists(x, box p, extras) => { Exists(x.clone(), bx!(p.in_world(world)), extras.in_world(world)) }
            ForAll(x, box p, extras) => { ForAll(x.clone(), bx!(p.in_world(world)), extras.in_world(world)) }
            Equals(x, y, extras) => { Equals(x.clone(), y.clone(), extras.in_world(world)) }
            DefinitelyExists(x, extras) => { DefinitelyExists(x.clone(), extras.in_world(world)) }
            Possible(box p, extras) => { Possible(bx!(p.in_world(world)), extras.in_world(world)) }
            Necessary(box p, extras) => { Necessary(bx!(p.in_world(world)), extras.in_world(world)) }
            InPast(box p, extras) => { InPast(bx!(p.in_world(world)), extras.in_world(world)) }
            InFuture(box p, extras) => { InFuture(bx!(p.in_world(world)), extras.in_world(world)) }
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
            DefinitelyExists(_, extras) => { extras.possible_world }
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
            DefinitelyExists(x, extras) => { DefinitelyExists(x.clone(), extras.with_sign(sign)) }
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
            DefinitelyExists(_, extras) => { extras.sign }
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
            DefinitelyExists(x, extras) => { DefinitelyExists(x.clone(), extras.with_is_hidden(is_hidden)) }
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
            DefinitelyExists(_, extras) => { extras.is_hidden }
            Possible(_, extras) => { extras.is_hidden }
            Necessary(_, extras) => { extras.is_hidden }
            InPast(_, extras) => { extras.is_hidden }
            InFuture(_, extras) => { extras.is_hidden }
            Comment(_) => { false }
        }
    }

    pub fn with_stripped_extras(&self) -> Formula
    {
        return match self
        {
            Atomic(p, _) => { Atomic(p.clone(), AtomicFormulaExtras::empty()) }
            Non(box p, _) => { Non(bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            And(box p, box q, _) => { And(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            Or(box p, box q, _) => { Or(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            Imply(box p, box q, _) => { Imply(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            BiImply(box p, box q, _) => { BiImply(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            StrictImply(box p, box q, _) => { StrictImply(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            Conditional(box p, box q, _) => { Conditional(bx!(p.with_stripped_extras()), bx!(q.with_stripped_extras()), FormulaExtras::empty()) }
            Exists(x, box p, _) => { Exists(x.clone(), bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            ForAll(x, box p, _) => { ForAll(x.clone(), bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            Equals(x, y, _) => { Equals(x.clone(), y.clone(), FormulaExtras::empty()) }
            DefinitelyExists(x, _) => { DefinitelyExists(x.clone(), FormulaExtras::empty()) }
            Possible(box p, _) => { Possible(bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            Necessary(box p, _) => { Necessary(bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            InPast(box p, _) => { InPast(bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
            InFuture(box p, _) => { InFuture(bx!(p.with_stripped_extras()), FormulaExtras::empty()) }
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
            Non(box p, _) => { p.get_predicate_arguments_of_atomic() }
            And(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Or(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Imply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            BiImply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            StrictImply(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Conditional(box p, box q, _) => { get_predicate_arguments_of_atomic_from_tuple((p, q)) }
            Exists(_, box p, _) => { p.get_predicate_arguments_of_atomic() }
            ForAll(_, box p, _) => { p.get_predicate_arguments_of_atomic() }
            Equals(_, _, _) => { None }
            DefinitelyExists(_, _) => { None }
            Possible(box p, _) => { p.get_predicate_arguments_of_atomic() }
            Necessary(box p, _) => { p.get_predicate_arguments_of_atomic() }
            InPast(box p, _) => { p.get_predicate_arguments_of_atomic() }
            InFuture(box p, _) => { p.get_predicate_arguments_of_atomic() }
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
                    output.insert(predicate_arg.clone());
                }
            }

            Exists(x, box p, _) =>
            {
                output.insert(x.clone());
                p.get_all_predicate_arguments_recursively(output);
            }
            ForAll(x, box p, _) =>
            {
                output.insert(x.clone());
                p.get_all_predicate_arguments_recursively(output);
            }
            Equals(x, y, _) =>
            {
                output.insert(x.clone());
                output.insert(y.clone());
            }
            DefinitelyExists(x, _) =>
            {
                output.insert(x.clone());
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
            ForAll(x, box p, _) =>
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