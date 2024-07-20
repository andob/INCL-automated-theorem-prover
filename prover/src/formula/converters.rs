use box_macro::bx;
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArgument, Sign};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Conditional, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};
use crate::logic::rule_apply_factory::RuleApplyFactory;

pub mod predicate_arg_instantiation;
mod extras_in_world;
mod extras_with_sign;
mod extras_with_is_hidden;

impl Formula
{
    pub fn instantiated(&self, factory : &mut RuleApplyFactory, x : &PredicateArgument, extras : &FormulaExtras) -> Formula
    {
        let mut instantiated_box = |p : &Box<Formula>| bx!(p.instantiated(factory, x, extras));

        return match self
        {
            Atomic(p, old_extras) =>
            {
                let new_extras = AtomicFormulaExtras
                {
                    predicate_args: old_extras.predicate_args.instantiated(factory, x),
                    possible_world: extras.possible_world,
                    is_hidden: old_extras.is_hidden,
                    sign: old_extras.sign,
                };

                return Atomic(p.clone(), new_extras);
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
            Possible(_, extras) => { extras.is_hidden }
            Necessary(_, extras) => { extras.is_hidden }
            InPast(_, extras) => { extras.is_hidden }
            InFuture(_, extras) => { extras.is_hidden }
            Comment(_) => { false }
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