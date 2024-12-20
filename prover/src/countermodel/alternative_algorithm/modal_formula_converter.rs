use anyhow::{anyhow, Result};
use box_macro::bx;
use itertools::Itertools;
use crate::countermodel::{CountermodelGraph, CountermodelGraphNode, CountermodelGraphVertex};
use crate::formula::{AtomicFormulaExtras, Formula, FormulaExtras, PossibleWorld};
use crate::formula::Formula::{And, Atomic, BiImply, Conditional, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};

impl Formula
{
    pub fn eliminate_modalities(&self, graph : &CountermodelGraph) -> Result<Formula>
    {
        return self.without_modalities(graph).without_falsum();
    }

    fn without_modalities(&self, graph : &CountermodelGraph) -> Formula
    {
        return match self
        {
            Possible(box Or(box p, box q, _), extras) =>
            {
                let possible_p = Possible(bx!(p.clone()), extras.clone());
                let possible_q = Possible(bx!(q.clone()), extras.clone());
                return Or(bx!(possible_p), bx!(possible_q), extras.clone());
            }

            Necessary(box And(box p, box q, _), extras) =>
            {
                let necessary_p = Necessary(bx!(p.clone()), extras.clone());
                let necessary_q = Necessary(bx!(q.clone()), extras.clone());
                return And(bx!(necessary_p), bx!(necessary_q), extras.clone());
            }

            Atomic(p_name, extras) =>
            {
                let new_p_name = format!("{}_{}", p_name, extras.possible_world.index);
                return Atomic(new_p_name, extras.clone());
            }

            Non(box p, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                return Non(bx!(p_prime), extras.clone())
            }

            And(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return And(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Or(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return Or(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Imply(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return Imply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            BiImply(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return BiImply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            StrictImply(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return StrictImply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Conditional(box p, box q, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                let q_prime = q.in_world(extras.possible_world).without_modalities(graph);
                return Conditional(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Exists(x, box p, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                return Exists(x.clone(), bx!(p_prime), extras.clone());
            }

            ForAll(x, box p, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                return ForAll(x.clone(), bx!(p_prime), extras.clone());
            }

            InPast(box p, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                return InPast(bx!(p_prime), extras.clone());
            }

            InFuture(box p, extras) =>
            {
                let p_prime = p.in_world(extras.possible_world).without_modalities(graph);
                return InFuture(bx!(p_prime), extras.clone());
            }

            Possible(box p, extras) |
            Necessary(box p, extras) =>
            {
                let accessible_worlds = graph.vertices.iter()
                    .filter(|vertex| vertex.from == extras.possible_world)
                    .map(|vertex| vertex.to)
                    .unique().collect_vec();

                if accessible_worlds.is_empty()
                {
                    return Formula::falsum();
                }

                if accessible_worlds.len() == 1
                {
                    return p.in_world(accessible_worlds[0]).without_modalities(graph);
                }

                let join_formulas : fn(Formula, Formula, FormulaExtras) -> Formula = if let Possible(..) = self
                    { |p, q, extras| Or(bx!(p), bx!(q), extras) }
                else { |p, q, extras| And(bx!(p), bx!(q), extras) };

                let mut joined_formulas = join_formulas(
                    p.in_world(accessible_worlds[0]).without_modalities(graph),
                    p.in_world(accessible_worlds[1]).without_modalities(graph),
                    extras.clone());

                for index in 2..accessible_worlds.len()
                {
                    joined_formulas = join_formulas(
                        joined_formulas.clone(),
                        p.in_world(accessible_worlds[index]).without_modalities(graph),
                        extras.clone());
                }

                return joined_formulas;
            }

            _ => self.clone()
        }
    }

    fn falsum() -> Formula
    {
        return Atomic(String::from('⊥'), AtomicFormulaExtras::empty());
    }

    fn without_falsum(&self) -> Result<Formula>
    {
        let mut formula = self.clone();
        let falsum = Formula::falsum();

        while formula.to_string().contains(&falsum.to_string())
        {
            // println!("{}", formula.to_string());
            formula = formula.without_falsum_impl();

            if formula == Formula::falsum()
            {
                return Err(anyhow!("Cannot eliminate falsum! Final result is falsum!"));
            }
        }

        // println!("{}", formula.to_string());
        return Ok(formula);
    }

    fn without_falsum_impl(&self) -> Formula
    {
        return match self
        {
            And(box p, box q@Atomic(..), _) if *q == Formula::falsum() => Formula::falsum(),
            And(box q@Atomic(..), box p, _) if *q == Formula::falsum() => Formula::falsum(),

            Or(box p, box q@Atomic(..), _) if *q == Formula::falsum() => p.clone(),
            Or(box q@Atomic(..), box p, _) if *q == Formula::falsum() => p.clone(),

            Non(box p, extras) => Non(bx!(p.without_falsum_impl()), extras.clone()),
            And(box p, box q, extras) => And(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            Or(box p, box q, extras) => Or(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            Imply(box p, box q, extras) => Imply(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            BiImply(box p, box q, extras) => BiImply(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            StrictImply(box p, box q, extras) => StrictImply(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            Conditional(box p, box q, extras) => Conditional(bx!(p.without_falsum_impl()), bx!(q.without_falsum_impl()), extras.clone()),
            Exists(x, box p, extras) => Exists(x.clone(), bx!(p.without_falsum_impl()), extras.clone()),
            ForAll(x, box p, extras) => ForAll(x.clone(), bx!(p.without_falsum_impl()), extras.clone()),
            InPast(box p, extras) => InPast(bx!(p.without_falsum_impl()), extras.clone()),
            InFuture(box p, extras) => InFuture(bx!(p.without_falsum_impl()), extras.clone()),
            Possible(box p, extras) => Possible(bx!(p.without_falsum_impl()), extras.clone()),
            Necessary(box p, extras) => Necessary(bx!(p.without_falsum_impl()), extras.clone()),

            _ => self.clone()
        }
    }
}
