use crate::formula::Formula;
use crate::formula::Sign::Minus;
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;
use minilp::{ComparisonOp, Error, OptimizationDirection, Problem as LinearProgram, Problem, Solution};

pub struct FuzzyLogicSemantics {}

impl Semantics for FuzzyLogicSemantics
{
    fn number_of_truth_values(&self) -> u8 { u8::MAX }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return formula.with_sign(Minus);
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, q : &Formula) -> bool
    {
        return false;
    }
}

impl FuzzyLogicSemantics
{
    pub fn linear_programming_demo(&self) -> bool
    {
        let mut linear_program = LinearProgram::new(OptimizationDirection::Maximize);

        let x = linear_program.add_var(1.0, (0.0, 1.0));
        let y = linear_program.add_var(1.0, (0.0, 1.0));
        let z = linear_program.add_var(1.0, (0.0, 1.0));
        let s = linear_program.add_var(1.0, (0.0, 1.0));
        let a = linear_program.add_var(1.0, (0.0, 1.0));
        let b = linear_program.add_var(1.0, (0.0, 1.0));

        // 0 + x + y < a
        // x + y - a < 0
        // [x 1] + [y 1] + [a -1] < 0
        // [x 1] + [y 1] + [a -1] <= -epsilon
        linear_program.add_constraint(&[(x, 1.0), (y, 1.0), (a, -1.0)], ComparisonOp::Le, -f64::EPSILON);

        // z >= a
        // z - a >= 0
        // [z 1] + [a -1] >= 0
        linear_program.add_constraint(&[(z, 1.0), (a, -1.0)], ComparisonOp::Ge, 0.0);

        // s + z < b
        // s + z - b < 0
        // [s 1] + [z 1] + [b -1] < 0
        linear_program.add_constraint(&[(s, 1.0), (z, 1.0), (b, -1.0)], ComparisonOp::Le, -f64::EPSILON);

        let is_contradictory = Self::is_contradictory(linear_program);
        println!("{}", is_contradictory);
        return is_contradictory;
    }

    fn is_contradictory(linear_program : Problem) -> bool
    {
        return match linear_program.solve()
        {
            Ok(solution) => solution.iter().any(|(_, value)| value.abs() == f64::EPSILON),
            Err(_) => true,
        };
    }
}
