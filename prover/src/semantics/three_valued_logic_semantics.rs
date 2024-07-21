use crate::formula::Formula;
use crate::formula::Formula::{Atomic, Non};
use crate::formula::Sign::{Minus, Plus};
use crate::semantics::Semantics;

pub struct ThreeValuedLogicSemantics
{
    contradiction_behaviour : ThreeValuedContradictionBehaviour
}

pub enum ThreeValuedContradictionBehaviour
{
    FormulaPlusWithFormulaMinus,
    FormulaPlusWithNonFormulaPlus,
    FormulaMinusWithNonFormulaMinus,
}

impl ThreeValuedLogicSemantics
{
    pub fn default() -> ThreeValuedLogicSemantics
    {
        let contradiction_behaviour = ThreeValuedContradictionBehaviour::FormulaPlusWithFormulaMinus;
        return ThreeValuedLogicSemantics { contradiction_behaviour };
    }

    pub fn with_contradiction_behaviour(contradiction_behaviour : ThreeValuedContradictionBehaviour) -> ThreeValuedLogicSemantics
    {
        return ThreeValuedLogicSemantics { contradiction_behaviour };
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
        //todo this does not account for predicate arguments

        return match self.contradiction_behaviour
        {
            ThreeValuedContradictionBehaviour::FormulaPlusWithFormulaMinus =>
            {
                match (p, q)
                {
                    (Atomic(p_name, _), Atomic(q_name, _)) |
                    (Non(box Atomic(p_name, _), _), Non(box Atomic(q_name, _), _))
                    if p.get_sign() * q.get_sign() == Minus /* p/q is +/- or -/+ */ =>
                    {
                        p_name == q_name &&
                        p.get_possible_world() == q.get_possible_world()
                    }

                    _ => { false }
                }
            }

            ThreeValuedContradictionBehaviour::FormulaPlusWithNonFormulaPlus =>
            {
                match (p, q)
                {
                    (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
                    (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
                    if p.get_sign() == Plus && q.get_sign() == Plus =>
                    {
                        p_name == q_name &&
                        p.get_possible_world() == q.get_possible_world()
                    }

                    _ => { false }
                }
            }

            ThreeValuedContradictionBehaviour::FormulaMinusWithNonFormulaMinus =>
            {
                match (p, q)
                {
                    (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
                    (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
                    if p.get_sign() == Minus && q.get_sign() == Minus =>
                    {
                        p_name == q_name &&
                        p.get_possible_world() == q.get_possible_world()
                    }

                    _ => { false }
                }
            }
        };
    }
}
