use crate::formula::Formula;
use crate::formula::Formula::{Atomic, Non};
use crate::formula::Sign::{Minus, Plus};
use crate::semantics::Semantics;

pub struct ThreeValuedLogicSemantics
{
    contradiction_behaviours : Vec<ThreeValuedContradictionBehaviour>
}

#[derive(Eq, PartialEq)]
pub enum ThreeValuedContradictionBehaviour
{
    FormulaPlusWithFormulaMinus,
    FormulaPlusWithNonFormulaPlus,
    FormulaMinusWithNonFormulaMinus,
}

impl ThreeValuedLogicSemantics
{
    pub fn new() -> ThreeValuedLogicSemantics
    {
        return ThreeValuedLogicSemantics
        {
            contradiction_behaviours: vec!
            [
                ThreeValuedContradictionBehaviour::FormulaPlusWithFormulaMinus
            ]
        };
    }
}

impl Semantics for ThreeValuedLogicSemantics
{
    //P could be true or false or unknown
    fn number_of_truth_values(&self) -> u8 { 3 }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return formula.with_sign(Minus);
    }

    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool
    {
        for contradiction_behaviour in &self.contradiction_behaviours
        {
            let is_contradiction = match contradiction_behaviour
            {
                ThreeValuedContradictionBehaviour::FormulaPlusWithFormulaMinus =>
                { self.are_formulas_contradictory_formula_plus_with_formula_minus(p, q) }

                ThreeValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus =>
                { self.are_formulas_contradictory_formula_plus_with_non_formula_plus(p, q) }

                ThreeValuedContradictionBehaviour::FormulaMinusWithNonFormulaMinus =>
                { self.are_formulas_contradictory_formula_minus_with_non_formula_minus(p, q) }
            };

            if is_contradiction
            {
                return true;
            }
        }

        return false;
    }
}

impl ThreeValuedLogicSemantics
{
    pub fn add_behaviour(&mut self, contradiction_behaviour : ThreeValuedContradictionBehaviour)
    {
        self.contradiction_behaviours.push(contradiction_behaviour);
    }

    fn are_formulas_contradictory_formula_plus_with_formula_minus(&self, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Atomic(q_name, _)) |
            (Non(box Atomic(p_name, _), _), Non(box Atomic(q_name, _), _))
            if p.get_sign() * q.get_sign() == Minus /* p/q is +/- or -/+ */ =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic() == q.get_predicate_arguments_of_atomic()
            }

            _ => { false }
        }
    }

    fn are_formulas_contradictory_formula_plus_with_non_formula_plus(&self, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
            if p.get_sign() == Plus && q.get_sign() == Plus =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic() == q.get_predicate_arguments_of_atomic()
            }

            _ => { false }
        }
    }

    fn are_formulas_contradictory_formula_minus_with_non_formula_minus(&self, p : &Formula, q : &Formula) -> bool
    {
        match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
            if p.get_sign() == Minus && q.get_sign() == Minus =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic() == q.get_predicate_arguments_of_atomic()
            }

            _ => { false }
        }
    }
}
