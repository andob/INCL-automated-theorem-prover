use crate::formula::Formula;
use crate::formula::Formula::{Atomic, Necessary, Non, Possible};
use crate::semantics::Semantics;

pub struct BinarySemantics {}
impl Semantics for BinarySemantics
{
    fn are_formulas_contradictory(&self, left : &Formula, right : &Formula) -> bool
    {
        //todo this does not account for predicate arguments
        //todo this does not account for possible worlds

        return match (left, right)
        {
            (Atomic(p, _), Non(box Atomic(q, _), _)) |
            (Non(box Atomic(p, _), _), Atomic(q, _)) |

            (Possible(box Atomic(p, _), _), Non(box Possible(box Atomic(q, _), _), _)) |
            (Non(box Possible(box Atomic(p, _), _), _), Possible(box Atomic(q, _), _)) |

            (Necessary(box Atomic(p, _), _), Non(box Necessary(box Atomic(q, _), _), _)) |
            (Non(box Necessary(box Atomic(p, _), _), _), Necessary(box Atomic(q, _), _))

            => { p==q }

            _ => { false }
        }
    }
}
