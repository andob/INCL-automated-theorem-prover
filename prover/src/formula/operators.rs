use std::ops::Mul;
use crate::formula::Sign::{Plus, Minus, PlusMinus};
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
            _ => { PlusMinus }
        }
    }
}