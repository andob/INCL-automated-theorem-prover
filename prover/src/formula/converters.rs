use box_macro::bx;
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld, PredicateArgument};
use crate::formula::Formula::{And, Atomic, BiImply, Comment, Exists, ForAll, Imply, Necessary, Non, Or, Possible, StrictImply};
use crate::logic::rule_apply_factory::RuleApplyFactory;

pub mod predicate_arg_instantiation;
mod extras_in_world;

impl Formula
{
    pub fn with(&self, extras : &FormulaExtras) -> Formula
    {
        return match self
        {
            Atomic(p, old_extras) =>
            {
                let new_extras = AtomicFormulaExtras
                {
                    predicate_args: old_extras.predicate_args.clone(),
                    possible_world: extras.possible_world,
                };

                return Atomic(p.clone(), new_extras);
            }

            Non(p, _) => { Non(p.clone(), extras.clone()) }
            And(p, q, _) => { And(p.clone(), q.clone(), extras.clone()) }
            Or(p, q, _) => { Or(p.clone(), q.clone(), extras.clone()) }
            Imply(p, q, _) => { Imply(p.clone(), q.clone(), extras.clone()) }
            BiImply(p, q, _) => { BiImply(p.clone(), q.clone(), extras.clone()) }
            StrictImply(p, q, _) => { StrictImply(p.clone(), q.clone(), extras.clone()) }
            Exists(x, p, _) => { Exists(x.clone(), p.clone(), extras.clone()) }
            ForAll(x, p, _) => { ForAll(x.clone(), p.clone(), extras.clone()) }
            Possible(p, _) => { Possible(p.clone(), extras.clone()) }
            Necessary(p, _) => { Necessary(p.clone(), extras.clone()) }
            Comment(payload) => { Comment(payload.clone()) }
        }
    }

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
                };

                return Atomic(p.clone(), new_extras);
            }

            Non(p, _) => { Non(instantiated_box(p), extras.clone()) }
            And(p, q, _) => { And(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Or(p, q, _) => { Or(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Imply(p, q, _) => { Imply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            BiImply(p, q, _) => { BiImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            StrictImply(p, q, _) => { StrictImply(instantiated_box(p), instantiated_box(q), extras.clone()) }
            Exists(x, p, _) => { Exists(x.clone(), instantiated_box(p), extras.clone()) }
            ForAll(x, p, _) => { ForAll(x.clone(), instantiated_box(p), extras.clone()) }
            Possible(p, _) => { Possible(instantiated_box(p), extras.clone()) }
            Necessary(p, _) => { Necessary(instantiated_box(p), extras.clone()) }
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
            Exists(x, p, extras) => { Exists(x.clone(), p.clone(), extras.in_world(possible_world)) }
            ForAll(x, p, extras) => { ForAll(x.clone(), p.clone(), extras.in_world(possible_world)) }
            Possible(p, extras) => { Possible(p.clone(), extras.in_world(possible_world)) }
            Necessary(p, extras) => { Necessary(p.clone(), extras.in_world(possible_world)) }
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
            Exists(_, _, extras) => { extras.possible_world }
            ForAll(_, _, extras) => { extras.possible_world }
            Possible(_, extras) => { extras.possible_world }
            Necessary(_, extras) => { extras.possible_world }
            Comment(_) => { PossibleWorld::zero() }
        }
    }
}

impl AtomicFormulaExtras
{
    pub fn merged_with(&self, other_extras : &FormulaExtras) -> AtomicFormulaExtras
    {
        return AtomicFormulaExtras
        {
            predicate_args: self.predicate_args.clone(),
            possible_world: other_extras.possible_world,
        }
    }
}
