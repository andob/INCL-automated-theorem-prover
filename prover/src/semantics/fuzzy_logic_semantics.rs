use crate::formula::Formula::{GreaterOrEqualThan, LessThan};
use crate::formula::Sign::Minus;
use crate::formula::{Formula, FuzzyTag, FuzzyTags};
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;
use minilp::{ComparisonOp, OptimizationDirection, Problem as LinearProgram, Variable};
use std::collections::BTreeMap;

pub struct FuzzyLogicSemantics {}

impl Semantics for FuzzyLogicSemantics
{
    fn number_of_truth_values(&self) -> u8 { u8::MAX }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        return formula.with_sign(Minus);
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, _ : &Formula) -> bool
    {
        //don't check for contradictions on formulas other than < and >=
        if !matches!(p, LessThan(..) | GreaterOrEqualThan(..)) { return false };

        let mut linear_program = LinearProgram::new(OptimizationDirection::Maximize);

        let variables = path.nodes.iter()
            .flat_map(|node| node.formula.get_fuzzy_tags().into_iter())
            .map(|fuzzy_tag| (fuzzy_tag, linear_program.add_var(1.0, (0.0, 1.0))))
            .collect::<BTreeMap<FuzzyTag, Variable>>();

        for formula in path.nodes.iter().map(|node| &node.formula)
        {
            if let LessThan(left, right, _) = formula
            {
                let vector = self.create_linear_program_constraint_vector(&variables, left, right);
                linear_program.add_constraint(vector, ComparisonOp::Le, -f64::EPSILON);
            }
            else if let GreaterOrEqualThan(left, right, _) = formula
            {
                let vector = self.create_linear_program_constraint_vector(&variables, left, right);
                linear_program.add_constraint(vector, ComparisonOp::Ge, 0.0);
            }
        }

        if let Ok(solution) = linear_program.solve()
        {
            //linear program has at least one insignificant solution => contradiction!
            return solution.iter().any(|(_, value)| value.abs() == f64::EPSILON);
        }

        //linear program has no solutions => contradiction!
        return true;
    }
}

impl FuzzyLogicSemantics
{
    fn create_linear_program_constraint_vector(&self,
        variables : &BTreeMap<FuzzyTag, Variable>,
        left_side_of_the_inequality : &FuzzyTags,
        right_side_of_the_inequality : &FuzzyTags,
    ) -> Box<[(Variable, f64)]>
    {
        let mut vector: Vec<(Variable, f64)> = Vec::new();

        for fuzzy_tag in left_side_of_the_inequality.iter()
        {
            vector.push((variables[fuzzy_tag], 1.0));
        }

        for fuzzy_tag in right_side_of_the_inequality.iter()
        {
            vector.push((variables[fuzzy_tag], -1.0));
        }

        return vector.into_boxed_slice();
    }
}
