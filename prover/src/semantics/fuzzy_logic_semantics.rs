use crate::logic::first_order_logic::FirstOrderLogicDomainType::VariableDomain;
use crate::formula::Formula::{GreaterOrEqualThan, LessThan};
use crate::formula::Sign::{Minus, Plus};
use crate::formula::{Formula, FuzzyTag, FuzzyTags};
use crate::semantics::Semantics;
use crate::tree::path::ProofTreePath;
use minilp::{ComparisonOp, OptimizationDirection, Problem as LinearProgram, Variable};
use std::collections::BTreeMap;
use itertools::Itertools;

const EPSILON : f64 = 0.001;

pub struct FuzzyLogicSemantics {}

impl Semantics for FuzzyLogicSemantics
{
    fn number_of_truth_values(&self) -> u8 { u8::MAX }

    fn reductio_ad_absurdum(&self, formula : &Formula) -> Formula
    {
        let initial_tags = FuzzyTags::new(vec![FuzzyTag::zero()]);

        return formula.with_sign(Minus).with_fuzzy_tags(initial_tags);
    }

    fn are_formulas_contradictory(&self, path : &ProofTreePath, p : &Formula, _ : &Formula) -> bool
    {
        //don't check for contradictions on formulas other than < and >=
        if !matches!(p, LessThan(..) | GreaterOrEqualThan(..)) { return false };

        let mut linear_program = LinearProgram::new(OptimizationDirection::Maximize);

        let variables = path.nodes.iter()
            .flat_map(|node| node.formula.get_fuzzy_tags().into_iter())
            .unique().map(|fuzzy_tag| (fuzzy_tag.get_variable_range(), fuzzy_tag.abs()))
            .map(|(range, fuzzy_tag)| (fuzzy_tag, linear_program.add_var(1.0, range)))
            .collect::<BTreeMap<FuzzyTag, Variable>>();
        if variables.is_empty() { return false };

        let mut number_of_constraints = 0usize;
        let mut has_non_strict_constraints = false;
        for formula in path.nodes.iter().map(|node| &node.formula)
        {
            if let LessThan(left, right, _) = formula
            {
                let vector = self.create_linear_program_constraint_vector(&variables, left, right);
                linear_program.add_constraint(vector, ComparisonOp::Le, -EPSILON);
                has_non_strict_constraints = true;
                number_of_constraints += 1;

            }
            else if let GreaterOrEqualThan(left, right, _) = formula
            {
                let vector = self.create_linear_program_constraint_vector(&variables, left, right);
                linear_program.add_constraint(vector, ComparisonOp::Ge, 0.0);
                number_of_constraints += 1;
            }
        }

        //we should have at least two inequalities
        if number_of_constraints < 2 { return false };

        /* todo hack: Linear programming technique does not work well with non-strict (<) inequalities.
            And the minilp library does not provide support for < inequalities, only <= inequalities.
            That's why x < 0 inequalities are implemented as x <= -EPSILON inequalities.
            However, this adds some ambiguity: sometimes, the program cannot distinguish between
            actual solutions and insignificant solutions. Is 1 - EPSILON a legit solution or not? */
        if path.domain_type == VariableDomain && has_non_strict_constraints
        {
            variables.iter()
                .filter(|(tag, _)| *tag != &FuzzyTag::zero() && *tag != &FuzzyTag::one())
                .map(|(_, variable)| ((*variable, 1.0), 1.0 - 2.0 * EPSILON))
                .for_each(|(x, y)| linear_program.add_constraint(&[x], ComparisonOp::Le, y));
        }

        if let Ok(solution) = linear_program.solve()
        {
            //linear program has at least one insignificant solution => contradiction!
            return variables.iter().map(|(_, variable)| solution[*variable])
                .any(|value| 1.0 - EPSILON <= value && value < 1.0);
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
            let variable = variables[&fuzzy_tag.abs()];
            let coefficient = if fuzzy_tag.sign == Plus { 1.0 } else { -1.0 };
            vector.push((variable, coefficient));
        }

        for fuzzy_tag in right_side_of_the_inequality.iter()
        {
            let variable = variables[&fuzzy_tag.abs()];
            let coefficient = if fuzzy_tag.sign == Plus { -1.0 } else { 1.0 };
            vector.push((variable, coefficient));
        }

        return vector.into_boxed_slice();
    }
}
