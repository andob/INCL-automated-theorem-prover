use std::ops::Mul;
use crate::formula::Sign::{Plus, Minus};
use crate::formula::Sign;

impl Mul for Sign
{
    type Output = Sign;
    fn mul(self, other : Self) -> Self::Output
    {
        return match (self, other)
        {
            (Plus, Plus) => { Plus }
            (Plus, Minus) => { Minus }
            (Minus, Plus) => { Minus }
            (Minus, Minus) => { Plus }
        }
    }
}