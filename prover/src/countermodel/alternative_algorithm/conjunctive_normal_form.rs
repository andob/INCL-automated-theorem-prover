use box_macro::bx;
use anyhow::{anyhow, Result};
use logicng::formulas::{EncodedFormula as LogicNGEncodedFormula, FormulaFactory as LogicNGFormulaFactory};
use crate::formula::Formula;
use crate::formula::Formula::{And, BiImply, Conditional, Exists, ForAll, Imply, InFuture, InPast, Necessary, Non, Or, Possible, StrictImply};
use crate::formula::notations::OperatorNotations;
use crate::formula::to_string::FormulaFormatOptions;

impl Formula
{
    pub fn to_conjunctive_normal_form(&self, logicng_formula_factory : &LogicNGFormulaFactory) -> Result<LogicNGEncodedFormula>
    {
        let mut formula_format_options = FormulaFormatOptions::default();
        formula_format_options.notations = OperatorNotations::LogicNGNotations;

        let logicng_input_formula = self.eliminate_equivalence().eliminate_implication();
        let logicng_input_string = logicng_input_formula.to_string_with_options(&formula_format_options);

        let logicng_parsed_formula = logicng_formula_factory.parse(logicng_input_string.as_str())?;
        let logicng_cnf = logicng_formula_factory.cnf_of(logicng_parsed_formula);
        return if logicng_cnf.is_falsum() { Err(anyhow!("CNF formula is falsum!")) }
        else { Ok(logicng_cnf) };
    }

    fn eliminate_implication(&self) -> Formula
    {
        return match self
        {
            Imply(box p, box q, extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                return Or(bx!(non_p), bx!(q.clone()), extras.clone());
            }

            Non(box Imply(box p, box q, _), extras) =>
            {
                let non_q = Non(bx!(q.clone()), extras.clone());
                return And(bx!(p.clone()), bx!(non_q), extras.clone());
            }

            Non(box p, extras) => Non(bx!(p.eliminate_implication()), extras.clone()),
            And(box p, box q, extras) => And(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            Or(box p, box q, extras) => Or(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            Imply(box p, box q, extras) => Imply(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            BiImply(box p, box q, extras) => BiImply(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            StrictImply(box p, box q, extras) => StrictImply(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            Conditional(box p, box q, extras) => Conditional(bx!(p.eliminate_implication()), bx!(q.eliminate_implication()), extras.clone()),
            Exists(x, box p, extras) => Exists(x.clone(), bx!(p.eliminate_implication()), extras.clone()),
            ForAll(x, box p, extras) => ForAll(x.clone(), bx!(p.eliminate_implication()), extras.clone()),
            InPast(box p, extras) => InPast(bx!(p.eliminate_implication()), extras.clone()),
            InFuture(box p, extras) => InFuture(bx!(p.eliminate_implication()), extras.clone()),
            Possible(box p, extras) => Possible(bx!(p.eliminate_implication()), extras.clone()),
            Necessary(box p, extras) => Necessary(bx!(p.eliminate_implication()), extras.clone()),

            _ => self.clone()
        }
    }

    fn eliminate_equivalence(&self) -> Formula
    {
        return match self
        {
            BiImply(box p, box q, extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_q = Non(bx!(q.clone()), extras.clone());
                let p_and_q = And(bx!(p.clone()), bx!(q.clone()), extras.clone());
                let non_p_and_non_q = And(bx!(non_p), bx!(non_q), extras.clone());
                return Or(bx!(p_and_q), bx!(non_p_and_non_q), extras.clone());
            }

            Non(box BiImply(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_q = Non(bx!(q.clone()), extras.clone());
                let p_or_q = Or(bx!(p.clone()), bx!(q.clone()), extras.clone());
                let non_p_or_non_q = Or(bx!(non_p), bx!(non_q), extras.clone());
                return And(bx!(p_or_q), bx!(non_p_or_non_q), extras.clone());
            }

            Non(box p, extras) => Non(bx!(p.eliminate_equivalence()), extras.clone()),
            And(box p, box q, extras) => And(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            Or(box p, box q, extras) => Or(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            Imply(box p, box q, extras) => Imply(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            BiImply(box p, box q, extras) => BiImply(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            StrictImply(box p, box q, extras) => StrictImply(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            Conditional(box p, box q, extras) => Conditional(bx!(p.eliminate_equivalence()), bx!(q.eliminate_equivalence()), extras.clone()),
            Exists(x, box p, extras) => Exists(x.clone(), bx!(p.eliminate_equivalence()), extras.clone()),
            ForAll(x, box p, extras) => ForAll(x.clone(), bx!(p.eliminate_equivalence()), extras.clone()),
            InPast(box p, extras) => InPast(bx!(p.eliminate_equivalence()), extras.clone()),
            InFuture(box p, extras) => InFuture(bx!(p.eliminate_equivalence()), extras.clone()),
            Possible(box p, extras) => Possible(bx!(p.eliminate_equivalence()), extras.clone()),
            Necessary(box p, extras) => Necessary(bx!(p.eliminate_equivalence()), extras.clone()),

            _ => self.clone()
        }
    }
}
