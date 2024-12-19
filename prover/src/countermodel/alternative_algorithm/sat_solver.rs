use std::collections::{BTreeMap, BTreeSet};
use logicng::datastructures::Model as LogicNGSatModel;
use logicng::solver::minisat::MiniSat as LogicNGMiniSat;
use logicng::formulas::{EncodedFormula as LogicNGEncodedFormula, FormulaFactory as LogicNGFormulaFactory};
use logicng::solver::minisat::sat::Tristate as LogicNGState;
use crate::countermodel::{CountermodelGraph, CountermodelGraphNode};

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
                    .map(|node| self.populate_node_with_values(node, &sat_model))
                    .collect::<BTreeSet<CountermodelGraphNode>>();

                return Some(CountermodelGraph
                {
                    nodes: nodes_with_attached_atomic_values,
                    vertices: graph.vertices.clone(),
                });
            }
        }

        return None;
    }

    fn populate_node_with_values(&self, node : &CountermodelGraphNode, sat_model : &LogicNGSatModel) -> CountermodelGraphNode
    {
        let mut atomic_values : BTreeMap<String, bool> = BTreeMap::new();
        let suffix = format!("_{}", node.possible_world.index);

        for variable in sat_model.pos()
        {
            let variable_name = variable.name(&self.formula_factory).to_string();
            if variable_name.ends_with(suffix.as_str())
            {
                let atomic_name = variable_name.strip_suffix(suffix.as_str()).unwrap_or_default();
                atomic_values.insert(atomic_name.to_string(), true);
            }
        }

        for variable in sat_model.neg()
        {
            let variable_name = variable.name(&self.formula_factory).to_string();
            if variable_name.ends_with(suffix.as_str())
            {
                let atomic_name = variable_name.strip_suffix(suffix.as_str()).unwrap_or_default();
                atomic_values.insert(atomic_name.to_string(), false);
            }
        }

        return CountermodelGraphNode
        {
            possible_world: node.possible_world,
            is_normal_world: node.is_normal_world,
            atomics: atomic_values,
        }
    }
}
