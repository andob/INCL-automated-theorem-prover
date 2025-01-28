mod graph_validator;
mod graph_generator;
mod modal_formula_converter;
mod conjunctive_normal_form;
mod sat_solver;
mod availability;
mod domain_generator;
mod first_order_formula_converter;
mod falsum_and_truthful;

use std::collections::BTreeSet;
use box_macro::bx;
use itertools::Itertools;
use logicng::formulas::{EncodedFormula as LogicNGEncodedFormula, FormulaFactory as LogicNGFormulaFactory};
use crate::countermodel::alternative_algorithm::availability::AlternativeCountermodelFinderAvailability;
use crate::countermodel::alternative_algorithm::domain_generator::CountermodelDomainGenerator;
use crate::countermodel::alternative_algorithm::graph_generator::CountermodelGraphGenerator;
use crate::countermodel::alternative_algorithm::sat_solver::SATSolver;
use crate::countermodel::CountermodelGraph;
use crate::formula::Formula::Non;
use crate::formula::{Formula, FormulaExtras, PredicateArgument};
use crate::tree::ProofTree;
use crate::utils::{get_config_value, CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES, CONFIG_KEY_MAX_COUNTERMODEL_DOMAIN_SIZE, CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES, CONFIG_KEY_MIN_COUNTERMODEL_DOMAIN_SIZE};

impl ProofTree
{
    pub fn find_countermodel_alt(&self) -> Option<CountermodelGraph>
    {
        //no countermodel if proof is correct
        if self.is_proof_correct { return None };

        //only implemented on classical logic, basic normal modal logics and their first-order constant domain counterparts
        let available_logic_names = AlternativeCountermodelFinderAvailability::get_available_logic_names();
        if !available_logic_names.contains(&self.problem.logic.get_name()) { return None };
        let logic = self.problem.logic.clone();

        let premises_and_non_conclusion = self.problem.premises.clone().into_iter()
            .chain(Some(Non(bx!(self.problem.conclusion.clone()), FormulaExtras::empty())))
            .collect::<Vec<Formula>>();

        let atomic_names = premises_and_non_conclusion.iter()
            .flat_map(|formula| formula.get_all_atomic_names())
            .collect::<BTreeSet<String>>();

        let predicate_arguments = premises_and_non_conclusion.iter()
            .flat_map(|formula| formula.get_all_predicate_arguments())
            .collect::<BTreeSet<PredicateArgument>>();

        let graph_generator = CountermodelGraphGenerator { logic:logic.clone(), atomic_names };
        let min_number_of_graph_nodes = get_config_value(CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES).unwrap_or(0);
        let max_number_of_graph_nodes = get_config_value(CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES).unwrap_or(u8::MAX);

        let domain_generator = CountermodelDomainGenerator { logic:logic.clone(), predicate_arguments };
        let min_domain_size = get_config_value(CONFIG_KEY_MIN_COUNTERMODEL_DOMAIN_SIZE).unwrap_or(1);
        let max_domain_size = get_config_value(CONFIG_KEY_MAX_COUNTERMODEL_DOMAIN_SIZE).unwrap_or(10);

        for number_of_graph_nodes in min_number_of_graph_nodes..=max_number_of_graph_nodes
        {
            for graph in graph_generator.generate_graphs(number_of_graph_nodes)
            {
                for domain in domain_generator.generate_domains(min_domain_size, max_domain_size)
                {
                    let formulas_without_quantifiers = premises_and_non_conclusion.iter()
                        .map(|formula| formula.eliminate_quantifiers(&domain))
                        .collect::<Vec<Formula>>();

                    let formulas_without_modalities = formulas_without_quantifiers.iter()
                        .map(|formula| formula.eliminate_modalities(&graph))
                        .collect::<Vec<Formula>>();

                    let logicng_formula_factory = LogicNGFormulaFactory::new();
                    let normalized_formulas = formulas_without_modalities.iter()
                        .map(|formula| formula.to_conjunctive_normal_form(&logicng_formula_factory))
                        .collect::<Vec<LogicNGEncodedFormula>>();

                    let normalized_formulas_as_strings = normalized_formulas.iter()
                        .map(|formula| formula.to_string(&logicng_formula_factory)).collect_vec();

                    let sat_solver = SATSolver::new(logicng_formula_factory, normalized_formulas);
                    if let Some(mut graph_with_atomic_values) = sat_solver.sat(&graph)
                    {
                        graph_with_atomic_values.comment = format!("Without quantifiers: {:?}\nWithout modalities: {:?}\nCNF: {:?}",
                            formulas_without_quantifiers, formulas_without_modalities, normalized_formulas_as_strings);

                        return Some(graph_with_atomic_values);
                    }
                }
            }
        }

        return None;
    }
}
