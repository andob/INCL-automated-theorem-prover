use crate::formula::Formula;
use crate::formula::Formula::{Atomic, DefinitelyExists, Equals, Non};
use crate::formula::Sign::{Minus, Plus};
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;

pub struct ManyValuedLogicSemantics
{
    truth_value_set_cardinality : TruthValueSetCardinality,
    contradiction_behaviours : Vec<ManyValuedContradictionBehaviour>,
}

enum TruthValueSetCardinality { Three, Four }

#[derive(Eq, PartialEq)]
pub enum ManyValuedContradictionBehaviour
{
    FormulaPlusWithFormulaMinus,
    FormulaPlusWithNonFormulaPlus,
    FormulaMinusWithNonFormulaMinus,
}

impl ManyValuedLogicSemantics
{
    pub fn with_three_values() -> ManyValuedLogicSemantics
    {
        return ManyValuedLogicSemantics
        {
            truth_value_set_cardinality: TruthValueSetCardinality::Three,
            contradiction_behaviours: vec!
            [
                ManyValuedContradictionBehaviour::FormulaPlusWithFormulaMinus
            ]
        };
    }

    pub fn with_four_values() -> ManyValuedLogicSemantics
    {
        return ManyValuedLogicSemantics
        {
            truth_value_set_cardinality: TruthValueSetCardinality::Four,
            contradiction_behaviours: vec!
            [
                ManyValuedContradictionBehaviour::FormulaPlusWithFormulaMinus
            ]
        };
    }
}

impl Semantics for ManyValuedLogicSemantics
{
    fn number_of_truth_values(&self) -> u8
    {
        return match self.truth_value_set_cardinality
        {
            //3-valued logic: true, false, unknown (neither true nor false)
            TruthValueSetCardinality::Three => 3,

            //4-valued logic: true, false, neither true nor false, both true and false
            TruthValueSetCardinality::Four => 4,
        };
    }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return formula.with_sign(Minus);
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        for contradiction_behaviour in &self.contradiction_behaviours
        {
            let is_contradiction = match contradiction_behaviour
            {
                ManyValuedContradictionBehaviour::FormulaPlusWithFormulaMinus =>
                { self.are_formulas_contradictory_formula_plus_with_formula_minus(path, p, q) }

                ManyValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus =>
                { self.are_formulas_contradictory_formula_plus_with_non_formula_plus(path, p, q) }

                ManyValuedContradictionBehaviour::FormulaMinusWithNonFormulaMinus =>
                { self.are_formulas_contradictory_formula_minus_with_non_formula_minus(path, p, q) }
            };

            if is_contradiction
            {
                return true;
            }
        }

        return false;
    }
}

impl ManyValuedLogicSemantics
{
    pub fn add_behaviour(&mut self, contradiction_behaviour : ManyValuedContradictionBehaviour)
    {
        self.contradiction_behaviours.push(contradiction_behaviour);
    }

    fn are_formulas_contradictory_formula_plus_with_formula_minus(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Atomic(q_name, _)) |
            (Non(box Atomic(p_name, _), _), Non(box Atomic(q_name, _), _))
            if p.get_sign() * q.get_sign() == Minus /* p/q is +/- or -/+ */ =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic_with_equivalences(path) ==
                q.get_predicate_arguments_of_atomic_with_equivalences(path)
            }

            (Equals(x, y, _), Equals(z, t, _))
            if p.get_sign() * q.get_sign() == Minus =>
            {
                ((x == z && y == t) || (x == t && y == z)) &&
                p.get_possible_world() == q.get_possible_world()
            }

            (DefinitelyExists(x, _), DefinitelyExists(y, _))
            if p.get_sign() * q.get_sign() == Minus =>
            {
                (x == y) && p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }

    fn are_formulas_contradictory_formula_plus_with_non_formula_plus(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
            if p.get_sign() == Plus && q.get_sign() == Plus =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic_with_equivalences(path) ==
                q.get_predicate_arguments_of_atomic_with_equivalences(path)
            }

            (Equals(x, y, _), Non(box Equals(z, t, _), _)) |
            (Non(box Equals(z, t, _), _), Equals(x, y, _))
            if p.get_sign() == Plus && q.get_sign() == Plus =>
            {
                ((x == z && y == t) || (x == t && y == z)) &&
                p.get_possible_world() == q.get_possible_world()
            }

            (DefinitelyExists(x, _), Non(box DefinitelyExists(y, _), _)) |
            (Non(box DefinitelyExists(y, _), _), DefinitelyExists(x, _))
            if p.get_sign() == Plus && q.get_sign() == Plus =>
            {
                (x == y) && p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }

    fn are_formulas_contradictory_formula_minus_with_non_formula_minus(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
            if p.get_sign() == Minus && q.get_sign() == Minus =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic_with_equivalences(path) ==
                q.get_predicate_arguments_of_atomic_with_equivalences(path)
            }

            (Equals(x, y, _), Non(box Equals(z, t, _), _)) |
            (Non(box Equals(z, t, _), _), Equals(x, y, _))
            if p.get_sign() == Minus && q.get_sign() == Minus =>
            {
                ((x == z && y == t) || (x == t && y == z)) &&
                p.get_possible_world() == q.get_possible_world()
            }

            (DefinitelyExists(x, _), Non(box DefinitelyExists(y, _), _)) |
            (Non(box DefinitelyExists(y, _), _), DefinitelyExists(x, _))
            if p.get_sign() == Minus && q.get_sign() == Minus =>
            {
                (x == y) && p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }
}
