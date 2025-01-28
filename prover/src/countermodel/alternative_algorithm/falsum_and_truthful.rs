use smol_str::SmolStr;
use crate::formula::{AtomicFormulaExtras, Formula};
use crate::formula::Formula::Atomic;

impl Formula
{
    pub fn falsum() -> Formula
    {
        return Atomic(SmolStr::from("@false"), AtomicFormulaExtras::empty());
    }

    pub fn truthful() -> Formula
    {
        return Atomic(SmolStr::from("@true"), AtomicFormulaExtras::empty());
    }
}
