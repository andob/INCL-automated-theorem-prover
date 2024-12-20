mod graph_validator;
mod graph_generator;
mod modal_formula_converter;
mod conjunctive_normal_form;
mod sat_solver;

use std::collections::BTreeSet;
use box_macro::bx;
use itertools::Itertools;
use logicng::formulas::{EncodedFormula as LogicNGEncodedFormula, FormulaFactory as LogicNGFormulaFactory};
use crate::countermodel::alternative_algorithm::graph_generator::CountermodelGraphGenerator;
use crate::countermodel::alternative_algorithm::sat_solver::SATSolver;
use crate::countermodel::CountermodelGraph;
use crate::formula::{Formula, FormulaExtras};
use crate::formula::Formula::Non;
use crate::formula::to_string::FormulaFormatOptions;
use crate::logic::Logic;
use crate::logic::normal_modal_logic::NormalModalLogic;
use crate::logic::propositional_logic::PropositionalLogic;
use crate::tree::ProofTree;
use crate::utils::{get_config_value, CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES, CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES};

impl ProofTree
{
    pub fn find_countermodel_alt(&self) -> Option<CountermodelGraph>
    {
        //no countermodel if proof is correct
        if self.is_proof_correct { return None };

        //only implemented on propositional logic and basic normal modal logics
        let available_logics =
        [
            (PropositionalLogic{}).get_name(),
            NormalModalLogic::K().get_name(),
            NormalModalLogic::T().get_name(),
            NormalModalLogic::B().get_name(),
            NormalModalLogic::S4().get_name(),
            NormalModalLogic::S5().get_name(),
        ];

        let logic = self.problem.logic.clone();
        if !available_logics.contains(&logic.get_name()) { return None };

        let premises_and_non_conclusion = self.problem.premises.clone().into_iter()
            .chain(Some(Non(bx!(self.problem.conclusion.clone()), FormulaExtras::empty())))
            .collect::<Vec<Formula>>();

        let atomic_names = premises_and_non_conclusion.iter()
            .flat_map(|formula| formula.get_all_atomic_names())
            .collect::<BTreeSet<String>>();

        let graph_generator = CountermodelGraphGenerator { logic:logic.clone(), atomic_names };
        let min_number_of_graph_nodes = get_config_value(CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES).unwrap_or(0);
        let max_number_of_graph_nodes = get_config_value(CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES).unwrap_or(u8::MAX);

        for number_of_graph_nodes in min_number_of_graph_nodes..=max_number_of_graph_nodes
        {
            for graph in graph_generator.generate_graphs(number_of_graph_nodes)
            {
                let logicng_formula_factory = LogicNGFormulaFactory::new();
                let formula_format_options = FormulaFormatOptions::recommended_for(&logic);

                let formulas_without_modalities = premises_and_non_conclusion.iter()
                    .flat_map(|formula| formula.eliminate_modalities(&graph).ok())
                    .collect::<Vec<Formula>>();

                let formulas_without_modalities_as_string = formulas_without_modalities.iter()
                    .map(|formula| formula.to_string_with_options(&formula_format_options)).join("\n");

                let normalized_formulas = formulas_without_modalities.iter()
                    .flat_map(|formula| formula.to_conjunctive_normal_form(&logicng_formula_factory))
                    .collect::<Vec<LogicNGEncodedFormula>>();

                let normalized_formulas_as_string = normalized_formulas.iter()
                    .map(|formula| formula.to_string(&logicng_formula_factory)).join("\n");

                if normalized_formulas.is_empty() { continue }
                if normalized_formulas.len() != premises_and_non_conclusion.len() { continue }

                let sat_solver = SATSolver::new(logicng_formula_factory, normalized_formulas);
                if let Some(mut graph_with_atomic_values) = sat_solver.sat(&graph)
                {
                    graph_with_atomic_values.comment = format!("Without modalities: {}\nCNF: {}",
                        formulas_without_modalities_as_string, normalized_formulas_as_string);
                    return Some(graph_with_atomic_values);
                }
            }
        }

        return None;
    }
}
