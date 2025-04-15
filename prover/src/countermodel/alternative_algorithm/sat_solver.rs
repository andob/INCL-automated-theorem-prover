use std::collections::BTreeSet;
use logicng::datastructures::Model as LogicNGSatModel;
use logicng::solver::minisat::MiniSat as LogicNGMiniSat;
use logicng::formulas::{EncodedFormula as LogicNGEncodedFormula, FormulaFactory as LogicNGFormulaFactory};
use logicng::solver::minisat::sat::Tristate as LogicNGState;
use smol_str::ToSmolStr;
use substring::Substring;
use crate::countermodel::{CountermodelGraph, CountermodelGraphNode};
use crate::formula::PossibleWorld;

pub struct SATSolver
{
    formula_factory : LogicNGFormulaFactory,
    formulas : Box<[LogicNGEncodedFormula]>,
}

impl SATSolver
{
    pub fn new(formula_factory : LogicNGFormulaFactory, formulas : Vec<LogicNGEncodedFormula>) -> SATSolver
    {
        return SATSolver { formula_factory:formula_factory, formulas:formulas.into_boxed_slice() };
    }

    pub fn sat(&self, graph : &CountermodelGraph) -> Option<CountermodelGraph>
    {
        let mut sat_solver = LogicNGMiniSat::new();
        sat_solver.add_all(&*self.formulas, &self.formula_factory);
        if sat_solver.sat() == LogicNGState::True
        {
            let variables = sat_solver.known_variables().into_boxed_slice();
            if let Some(sat_model) = sat_solver.model(Some(&*variables))
            {
                let nodes_with_attached_atomic_values = graph.nodes.iter()
                    .map(|node| self.new_node_with_atomic_values(node, &sat_model))
                    .collect::<BTreeSet<CountermodelGraphNode>>();

                return Some(CountermodelGraph
                {
                    nodes: nodes_with_attached_atomic_values,
                    vertices: graph.vertices.clone(),
                    was_built_from_modality_graph: false,
                    comment: String::new(),
                });
            }
        }

        return None;
    }

    fn new_node_with_atomic_values(&self, node : &CountermodelGraphNode, sat_model : &LogicNGSatModel) -> CountermodelGraphNode
    {
        let mut node = node.clone();

        let suffix = format!("_{}", node.possible_world.index);

        for variable in sat_model.pos()
        {
            let variable_name = variable.name(&self.formula_factory).to_string();
            if variable_name.ends_with(suffix.as_str())
            {
                let atomic_name = self.parse_atomic_name(&variable_name, node.possible_world);
                node.atomics.insert(atomic_name, true);
            }
        }

        for variable in sat_model.neg()
        {
            let variable_name = variable.name(&self.formula_factory).to_string();
            if variable_name.ends_with(suffix.as_str())
            {
                let atomic_name = self.parse_atomic_name(&variable_name, node.possible_world);
                node.atomics.insert(atomic_name, false);
            }
        }

        return node;
    }

    fn parse_atomic_name(&self, raw_name : &String, possible_world : PossibleWorld) -> String
    {
        let suffix = format!("_{}", possible_world.index);
        let raw_name = raw_name.strip_suffix(suffix.as_str()).unwrap_or_default();

        if raw_name.contains('_')
        {
            //first order logic: this is a predicate
            let index = raw_name.find('_').unwrap_or_default();
            let predicate_name = raw_name.substring(0, index);
            let predicate_args = raw_name.substring(index+1, raw_name.len()).replace('_', ",");
            return format!("{}[{}]", predicate_name, predicate_args);
        }

        //propositional logic: this is a proposition
        return raw_name.to_string();
    }
}
