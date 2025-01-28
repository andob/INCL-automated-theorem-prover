use std::collections::BTreeSet;
use box_macro::bx;
use itertools::Itertools;
use smol_str::{format_smolstr, SmolStr};
use crate::formula::{Formula, PredicateArguments};
use crate::formula::Formula::{And, Atomic, BiImply, Equals, Exists, ForAll, Imply, Necessary, Non, Or, Possible, StrictImply};

impl Formula
{
    pub fn eliminate_quantifiers(&self, domain : &BTreeSet<SmolStr>) -> Formula
    {
        let domain_as_vec = domain.iter().cloned().collect_vec();
        return self.without_quantifiers(&domain_as_vec);
    }

    fn without_quantifiers(&self, domain : &Vec<SmolStr>) -> Formula
    {
        return match self
        {
            Exists(x, box Or(box p, box q, _), extras) =>
            {
                let exists_px = Exists(x.clone(), bx!(p.clone()), extras.clone());
                let exists_qx = Exists(x.clone(), bx!(q.clone()), extras.clone());
                return Or(bx!(exists_px), bx!(exists_qx), extras.clone());
            }

            ForAll(x, box And(box p, box q, _), extras) =>
            {
                let forall_px = ForAll(x.clone(), bx!(p.clone()), extras.clone());
                let forall_qx = ForAll(x.clone(), bx!(q.clone()), extras.clone());
                return And(bx!(forall_px), bx!(forall_qx), extras.clone());
            }

            p@Atomic(..) if *p == Formula::falsum() => Formula::falsum(),
            p@Atomic(..) if *p == Formula::truthful() => Formula::truthful(),

            Atomic(p_name, extras) if !extras.predicate_args.is_empty() =>
            {
                let new_p_name = format_smolstr!("{}_{}", p_name, extras.predicate_args
                    .iter().map(|arg| &arg.object_name).join("_"));

                let mut new_extras = extras.clone();
                new_extras.predicate_args = PredicateArguments::empty();
                return Atomic(new_p_name, new_extras);
            }

            Equals(x, y, _) if x == y => Formula::truthful(),
            Equals(x, y, _) if x != y => Formula::falsum(),
            Non(box Equals(x, y, _), _) if x == y => Formula::falsum(),
            Non(box Equals(x, y, _), _) if x != y => Formula::truthful(),

            Non(box p, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                return Non(bx!(p_prime), extras.clone())
            }

            And(box p, box q, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                let q_prime = q.without_quantifiers(&domain);
                return And(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Or(box p, box q, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                let q_prime = q.without_quantifiers(&domain);
                return Or(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Imply(box p, box q, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                let q_prime = q.without_quantifiers(&domain);
                return Imply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            BiImply(box p, box q, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                let q_prime = q.without_quantifiers(&domain);
                return BiImply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            StrictImply(box p, box q, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                let q_prime = q.without_quantifiers(&domain);
                return StrictImply(bx!(p_prime), bx!(q_prime), extras.clone());
            }

            Possible(box p, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                return Possible(bx!(p_prime), extras.clone())
            }

            Necessary(box p, extras) =>
            {
                let p_prime = p.without_quantifiers(&domain);
                return Necessary(bx!(p_prime), extras.clone())
            }

            Exists(x, box p, extras) =>
            {
                if domain.is_empty()
                {
                    return Formula::falsum();
                }

                if domain.len() == 1
                {
                    let (binded_p, _) = p.binded(x, domain[0].clone(), extras);
                    return binded_p.without_quantifiers(&domain);
                }

                let (binded_p0, _) = p.binded(x, domain[0].clone(), extras);
                let (binded_p1, _) = p.binded(x, domain[1].clone(), extras);
                let binded_p0 = binded_p0.without_quantifiers(&domain);
                let binded_p1 = binded_p1.without_quantifiers(&domain);
                let mut joined_formulas = Or(bx!(binded_p0), bx!(binded_p1), extras.clone());

                for index in 2..domain.len()
                {
                    let (binded_pi, _) = p.binded(x, domain[index].clone(), extras);
                    let binded_pi = binded_pi.without_quantifiers(&domain);
                    joined_formulas = Or(bx!(joined_formulas.clone()), bx!(binded_pi), extras.clone());
                }

                return joined_formulas;
            }

            ForAll(x, box p, extras) =>
            {
                if domain.is_empty()
                {
                    return Formula::truthful();
                }

                if domain.len() == 1
                {
                    let (binded_p, _) = p.binded(x, domain[0].clone(), extras);
                    return binded_p.without_quantifiers(&domain);
                }

                let (binded_p0, _) = p.binded(x, domain[0].clone(), extras);
                let (binded_p1, _) = p.binded(x, domain[1].clone(), extras);
                let binded_p0 = binded_p0.without_quantifiers(&domain);
                let binded_p1 = binded_p1.without_quantifiers(&domain);
                let mut joined_formulas = And(bx!(binded_p0), bx!(binded_p1), extras.clone());

                for index in 2..domain.len()
                {
                    let (binded_pi, _) = p.binded(x, domain[index].clone(), extras);
                    let binded_pi = binded_pi.without_quantifiers(&domain);
                    joined_formulas = And(bx!(joined_formulas.clone()), bx!(binded_pi), extras.clone());
                }

                return joined_formulas;
            }

            _ => self.clone()
        }
    }
}
