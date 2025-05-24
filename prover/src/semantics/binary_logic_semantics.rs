use box_macro::bx;
use crate::formula::{Formula, FormulaExtras};
use crate::formula::Formula::{Atomic, Necessary, Non, Possible, BiImply, Equals, DefinitelyExists};
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;

pub struct BinaryLogicSemantics {}
impl Semantics for BinaryLogicSemantics
{
    //P could be true or false
    fn number_of_truth_values(&self) -> u8 { 2 }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return Non(bx!(formula.clone()), FormulaExtras::empty());
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        return match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _))
            =>
            {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic_with_equivalences(path) ==
                q.get_predicate_arguments_of_atomic_with_equivalences(path)
            }

            (BiImply(box Atomic(n1, _), box Atomic(n2, _), _), Non(box BiImply(box Atomic(n3, _), box Atomic(n4, _), _), _)) |
            (Non(box BiImply(box Atomic(n1, _), box Atomic(n2, _), _), _), BiImply(box Atomic(n3, _), box Atomic(n4, _), _))
            =>
            {
                n1 == n3 && n2 == n4 &&
                p.get_possible_world() == q.get_possible_world() &&
                p.get_predicate_arguments_of_atomic_with_equivalences(path) ==
                q.get_predicate_arguments_of_atomic_with_equivalences(path)
            }

            (Equals(x, y, _), Non(box Equals(z, t, _), _)) |
            (Non(box Equals(x, y, _), _), Equals(z, t, _))
            =>
            {
                ((x == z && y == t) || (x == t && y == z)) &&
                p.get_possible_world() == q.get_possible_world()
            }

            (DefinitelyExists(x, _), Non(box DefinitelyExists(y, _), _)) |
            (Non(box DefinitelyExists(x, _), _), DefinitelyExists(y, _))
            =>
            {
                (x == y) && p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }
}
