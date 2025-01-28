use smol_str::SmolStr;
use crate::formula::{AtomicFormulaExtras, Formula};

impl Formula
{
    pub fn truth() -> Formula
    {
        return Formula::Atomic(SmolStr::from("@true"), AtomicFormulaExtras::empty());
    }

    pub fn falsum() -> Formula
    {
        return Formula::Atomic(SmolStr::from("@false"), AtomicFormulaExtras::empty());
    }
}
