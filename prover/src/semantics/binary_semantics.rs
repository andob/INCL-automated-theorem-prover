use box_macro::bx;

use crate::formula::{Formula, FormulaExtras};
use crate::formula::Formula::{Atomic, Necessary, Non, Possible, BiImply};
use crate::semantics::Semantics;

pub struct BinarySemantics {}
impl Semantics for BinarySemantics
{
    fn negate(&self, p : &Formula, extras : &FormulaExtras) -> Formula
    {
        return Non(bx!(p.with(extras)), extras.clone());
    }

    fn are_formulas_contradictory(&self, p : &Formula, q : &Formula) -> bool
    {
        //todo this does not account for predicate arguments

        return match (p, q)
        {
            (Atomic(p_name, _), Non(box Atomic(q_name, _), _)) |
            (Non(box Atomic(p_name, _), _), Atomic(q_name, _)) |

            (Possible(box Atomic(p_name, _), _), Non(box Possible(box Atomic(q_name, _), _), _)) |
            (Non(box Possible(box Atomic(p_name, _), _), _), Possible(box Atomic(q_name, _), _)) |

            (Necessary(box Atomic(p_name, _), _), Non(box Necessary(box Atomic(q_name, _), _), _)) |
            (Non(box Necessary(box Atomic(p_name, _), _), _), Necessary(box Atomic(q_name, _), _))

            => {
                p_name == q_name &&
                p.get_possible_world() == q.get_possible_world()
            }

            (BiImply(box Atomic(n1, _), box Atomic(n2, _), _), Non(box BiImply(box Atomic(n3, _), box Atomic(n4, _), _), _)) |
            (Non(box BiImply(box Atomic(n1, _), box Atomic(n2, _), _), _), BiImply(box Atomic(n3, _), box Atomic(n4, _), _))

            => {
                n1 == n3 && n2 == n4 &&
                p.get_possible_world() == q.get_possible_world()
            }

            _ => { false }
        }
    }
}
