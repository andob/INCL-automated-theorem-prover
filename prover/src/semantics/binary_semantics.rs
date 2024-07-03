use box_macro::bx;
use crate::formula::{Formula, FormulaExtras};
use crate::formula::Formula::{Atomic, Necessary, Non, Possible};
use crate::semantics::Semantics;

pub struct BinarySemantics {}
impl Semantics for BinarySemantics
{
    fn negate(&self, p : &Formula, extras : &FormulaExtras) -> Formula
    {
        return Non(bx!(p.with(extras)), extras.clone());
    }

    fn are_formulas_contradictory(&self, left : &Formula, right : &Formula) -> bool
    {
        return match (left, right)
        {
            (Atomic(p, p_extras), Non(box Atomic(q, q_extras), _)) |
            (Non(box Atomic(p, p_extras), _), Atomic(q, q_extras)) |

            (Possible(box Atomic(p, p_extras), _), Non(box Possible(box Atomic(q, q_extras), _), _)) |
            (Non(box Possible(box Atomic(p, p_extras), _), _), Possible(box Atomic(q, q_extras), _)) |

            (Necessary(box Atomic(p, p_extras), _), Non(box Necessary(box Atomic(q, q_extras), _), _)) |
            (Non(box Necessary(box Atomic(p, p_extras), _), _), Necessary(box Atomic(q, q_extras), _))

            => {
                p == q
                && p_extras.possible_world == q_extras.possible_world
                && p_extras.predicate_args.len() == q_extras.predicate_args.len()
                && p_extras.predicate_args.iter().zip(q_extras.predicate_args.iter())
                    .all(|(x, y)| x.type_name == y.type_name)
            }

            _ => { false }
        }
    }
}
