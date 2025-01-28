use crate::countermodel::CountermodelGraph;
use crate::formula::Formula::{And, Atomic, BiImply, Exists, ForAll, Imply, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::Formula;
use box_macro::bx;
use itertools::Itertools;
use smol_str::format_smolstr;

impl Formula
{
    pub fn eliminate_modalities(&self, graph : &CountermodelGraph) -> Formula
    {
        return self.without_modalities(graph);
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

            p@Atomic(..) if *p == Formula::falsum() => Formula::falsum(),
            p@Atomic(..) if *p == Formula::truthful() => Formula::truthful(),

            Atomic(p_name, extras) =>
            {
                let new_p_name = format_smolstr!("{}_{}", p_name, extras.possible_world.index);
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

            Possible(box p, extras) =>
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

                let p_in_world0 = p.in_world(accessible_worlds[0]).without_modalities(graph);
                let p_in_world1 = p.in_world(accessible_worlds[1]).without_modalities(graph);
                let mut joined_formulas = Or(bx!(p_in_world0), bx!(p_in_world1), extras.clone());

                for index in 2..accessible_worlds.len()
                {
                    let p_in_world_i = p.in_world(accessible_worlds[index]).without_modalities(graph);
                    joined_formulas = Or(bx!(joined_formulas.clone()), bx!(p_in_world_i), extras.clone());
                }

                return joined_formulas;
            }

            Necessary(box p, extras) =>
            {
                let accessible_worlds = graph.vertices.iter()
                    .filter(|vertex| vertex.from == extras.possible_world)
                    .map(|vertex| vertex.to)
                    .unique().collect_vec();

                if accessible_worlds.is_empty()
                {
                    return Formula::truthful();
                }

                if accessible_worlds.len() == 1
                {
                    return p.in_world(accessible_worlds[0]).without_modalities(graph);
                }

                let p_in_world0 = p.in_world(accessible_worlds[0]).without_modalities(graph);
                let p_in_world1 = p.in_world(accessible_worlds[1]).without_modalities(graph);
                let mut joined_formulas = And(bx!(p_in_world0), bx!(p_in_world1), extras.clone());

                for index in 2..accessible_worlds.len()
                {
                    let p_in_world_i = p.in_world(accessible_worlds[index]).without_modalities(graph);
                    joined_formulas = And(bx!(joined_formulas.clone()), bx!(p_in_world_i), extras.clone());
                }

                return joined_formulas;
            }

            _ => self.clone()
        }
    }
}
