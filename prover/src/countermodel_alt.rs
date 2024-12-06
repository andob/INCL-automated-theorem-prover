use crate::countermodel::{CountermodelGraph, CountermodelGraphNode, CountermodelGraphVertex};
use crate::formula::{Formula, PossibleWorld};
use crate::logic::normal_modal_logic::NormalModalLogic;
use crate::logic::propositional_logic::PropositionalLogic;
use crate::logic::Logic;
use crate::tree::ProofTree;
use crate::problem::Problem;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::One;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;
use std::rc::Rc;
use std::str::FromStr;

const KEY_MIN_NO_GRAPH_NODES : &str = "min_countermodel_graph_nodes";
const KEY_MAX_NO_GRAPH_NODES : &str = "max_countermodel_graph_nodes";

impl ProofTree
{
    pub fn find_countermodel_alt(&self) -> Option<CountermodelGraph>
    {
        //no countermodel if proof is correct
        if self.is_proof_correct { return None };

        //todo de implementat un SAT solver
        //https://en.wikipedia.org/wiki/SAT_solver
        //https://crates.io/crates/splr
        //https://docs.rs/sat/latest/sat/
        //https://github.com/jix/varisat
        //https://github.com/sarsko/CreuSAT

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

        let atomic_names = self.problem.premises.iter()
            .chain(Some(&self.problem.conclusion).into_iter())
            .flat_map(|formula| formula.get_all_atomic_names())
            .filter(|p| !p.starts_with(KEY_MIN_NO_GRAPH_NODES))
            .filter(|p| !p.starts_with(KEY_MAX_NO_GRAPH_NODES))
            .collect::<BTreeSet<String>>();

        let graph_generator = CountermodelGraphGenerator { logic:logic.clone(), atomic_names };
        let min_number_of_graph_nodes = get_config_arg(&self.problem, KEY_MIN_NO_GRAPH_NODES, 0);
        let max_number_of_graph_nodes = get_config_arg(&self.problem, KEY_MAX_NO_GRAPH_NODES, u8::MAX);

        for number_of_graph_nodes in min_number_of_graph_nodes..=max_number_of_graph_nodes
        {
            let premises = self.problem.premises.clone();
            let conclusion = self.problem.conclusion.clone();

            let result = graph_generator.generate_graphs_with_values(&logic, number_of_graph_nodes, Box::new(move |graph|
            {
                let are_all_premises_true = premises.iter()
                    .filter(|p| !p.to_string().starts_with(KEY_MIN_NO_GRAPH_NODES))
                    .filter(|p| !p.to_string().starts_with(KEY_MAX_NO_GRAPH_NODES))
                    .all(|premise| graph.evaluate(premise, PossibleWorld::zero()));

                let is_conclusion_true = graph.evaluate(&conclusion, PossibleWorld::zero());

                return if are_all_premises_true && !is_conclusion_true { Some(graph) } else { None };
            }));

            let should_stop_iterating = result.is_some();
            if should_stop_iterating { return result };
        }

        return None;
    }
}

fn get_config_arg(problem : &Problem, key : &str, default : u8) -> u8
{
    let key_with_comma = format!("{}:", key);

    let value_as_string = problem.premises.iter()
        .filter_map(|p| if let Formula::Atomic(payload, _) = p { Some(payload) } else { None })
        .find(|payload| payload.starts_with(key_with_comma.as_str()))
        .map(|payload| payload.trim_start_matches(key_with_comma.as_str()).to_string())
        .unwrap_or_default();

    return u8::from_str(value_as_string.as_str()).unwrap_or(default);
}

pub struct CountermodelGraphGenerator
{
    pub logic : Rc<dyn Logic>,
    pub atomic_names : BTreeSet<String>,
}

impl CountermodelGraphGenerator
{
    pub fn generate_atomic_values(&self) -> Vec<BTreeMap<String, bool>>
    {
        let mut generated_result : Vec<BTreeMap<String, bool>> = Vec::new();

        let number_of_truth_values = self.logic.get_semantics().number_of_truth_values();
        for code in 0..(number_of_truth_values as u64).pow(self.atomic_names.len() as u32)
        {
            let mut atomics_with_values : BTreeMap<String, bool> = BTreeMap::new();

            for (index, atomic_name) in self.atomic_names.iter().enumerate()
            {
                let atomic_value = ((code & (1u64 << index)) >> index) == 1;
                atomics_with_values.insert(atomic_name.to_string(), atomic_value);
            }

            generated_result.push(atomics_with_values);
        }

        return generated_result;
    }

    pub fn generate_graphs<R>(&self, logic : &Rc<dyn Logic>, number_of_nodes : u8,
        callback : Box<dyn Fn(CountermodelGraph) -> Option<R>>) -> Option<R> where R : 'static
    {
        let mut code = BigUint::ZERO;
        while code < BigUint::from(2u8).pow(number_of_nodes.pow(2) as u32)
        {
            let mut graph = CountermodelGraph::new();

            for world_index in 0..number_of_nodes
            {
                graph.nodes.insert(CountermodelGraphNode
                {
                    possible_world: PossibleWorld { index: world_index },
                    is_normal_world: true, atomics: BTreeMap::new(),
                });
            }

            let mut bit_index = 0u64;
            for from_world_index in 0..number_of_nodes
            {
                for to_world_index in 0..number_of_nodes
                {
                    if code.bit(bit_index)
                    {
                        graph.vertices.insert(CountermodelGraphVertex
                        {
                            from: PossibleWorld { index: from_world_index },
                            to: PossibleWorld { index: to_world_index },
                            tags: Vec::new(),
                        });
                    }

                    bit_index += 1;
                }
            }

            if graph.validate(logic).is_ok()
            {
                let result = callback(graph);
                let should_stop_generating = result.is_some();
                if should_stop_generating { return result };
            }

            code = code.add(BigUint::one());
        }

        return None;
    }

    pub fn combine_atomic_values_to_matrix(&self, atomic_values : Vec<BTreeMap<String, bool>>, dimension : u8) -> Vec<Vec<BTreeMap<String, bool>>>
    {
        if dimension == 0 { return Vec::new() };

        if dimension == 1
        {
            //atomic_values vector transposed
            return atomic_values.into_iter()
                .map(|values| vec![values]).collect();
        };

        //generate cartesian product
        let mut matrix : Vec<Vec<BTreeMap<String, bool>>> = atomic_values.clone()
            .into_iter().cartesian_product(atomic_values.clone())
            .map(|(a, b)| vec![a, b]).collect();
        if dimension == 2 { return matrix };

        for _ in 3..=dimension
        {
            //generate cartesian product again
            matrix = atomic_values.clone()
                .into_iter().cartesian_product(matrix)
                .map(|(a, mut b)|
                {
                    let mut result : Vec<BTreeMap<String, bool>> = Vec::new();
                    result.push(a); result.append(&mut b);
                    return result;
                }).collect();
        }

        return matrix;
    }

    pub fn generate_graphs_with_values<R>(&self, logic : &Rc<dyn Logic>, number_of_nodes : u8,
        callback : Box<dyn Fn(CountermodelGraph) -> Option<R>>) -> Option<R> where R : 'static
    {
        let atomic_values = self.generate_atomic_values();
        let atomic_values_combinations = self.combine_atomic_values_to_matrix(atomic_values, number_of_nodes);

        return self.generate_graphs(logic, number_of_nodes, Box::new(move |graph|
        {
            for atomic_values_combination in &atomic_values_combinations
            {
                let mut graph_nodes_with_attached_values : BTreeSet<CountermodelGraphNode> = BTreeSet::new();

                for (world_index, atomic_values) in atomic_values_combination.into_iter().enumerate()
                {
                    let mut graph_node = graph.nodes.iter()
                        .find(|node| node.possible_world.index == world_index as u8)
                        .cloned().unwrap();

                    graph_node.atomics = atomic_values.clone();
                    graph_nodes_with_attached_values.insert(graph_node);
                }

                let graph_with_attached_values = CountermodelGraph
                {
                    nodes: graph_nodes_with_attached_values,
                    vertices: graph.vertices.clone(),
                };

                let result = callback(graph_with_attached_values);
                let should_stop_generating = result.is_some();
                if should_stop_generating { return result };
            }

            return None;
        }));
    }
}

impl CountermodelGraph
{
    pub fn validate(&self, logic : &Rc<dyn Logic>) -> Result<()>
    {
        let mut validation_message = String::new();
        let mut is_valid = true;

        if self.nodes.is_empty()
        {
            validation_message.push_str("Invalid graph: no nodes!");
            is_valid = false;
        }

        if self.vertices.is_empty()
        {
            validation_message.push_str("Invalid graph: no vertices!");
            is_valid = false;
        }

        if !self.nodes.iter()
            .filter(|node| node.possible_world.index>0)
            .all(|node| self.vertices.iter().any(|vertex|
                vertex.from != vertex.to && vertex.to == node.possible_world))
        {
            validation_message.push_str("Invalid graph: completely disconnected worlds!");
            is_valid = false;
        }

        if logic.get_name().is_normal_modal_logic()
        {
            let logic = logic.cast_to::<NormalModalLogic>().unwrap();

            if logic.is_reflexive && !self.nodes.iter()
                .all(|node| self.vertices.iter().any(|vertex|
                    vertex.from == node.possible_world && vertex.from == vertex.to))
            {
                validation_message.push_str("Invalid graph: not reflexive!");
                is_valid = false;
            }

            if logic.is_symmetric && !self.vertices.iter()
                .filter(|v1| v1.from != v1.to)
                .all(|v1| self.vertices.iter()
                    .filter(|v2| v2.from != v2.to)
                    .any(|v2| v1.from == v2.to && v1.to == v2.from))
            {
                validation_message.push_str("Invalid graph: not symmetric!");
                is_valid = false;
            }

            if logic.is_transitive && !self.vertices.iter()
                .cartesian_product(self.vertices.iter())
                .filter(|(v1, v2)|
                    v1.from != v1.to && v2.from != v2.to && v2.from == v1.to)
                .all(|(v1, v2)| self.vertices.iter()
                    .any(|v3| v3.from == v1.from && v3.to == v2.to))
            {
                validation_message.push_str("Invalid graph: not transitive!");
                is_valid = false;
            }
        }

        return if is_valid { Ok(()) } else { Err(anyhow!(validation_message)) };
    }

    pub fn evaluate(&self, formula : &Formula, possible_world : PossibleWorld) -> bool
    {
        return match formula
        {
            Formula::Atomic(p, _) =>
            {
                let p_value = self.nodes.iter()
                    .find(|node| node.possible_world == possible_world)
                    .map(|node| node.atomics.get(p))
                    .unwrap_or_default().cloned().unwrap_or_default();

                return p_value;
            }

            Formula::Non(box p, _) =>
            {
                return !self.evaluate(p, possible_world);
            }

            Formula::And(box p, box q, _) =>
            {
                let p_value = self.evaluate(p, possible_world);
                let q_value = self.evaluate(q, possible_world);
                return p_value && q_value;
            }

            Formula::Or(box p, box q, _) =>
            {
                let p_value = self.evaluate(p, possible_world);
                let q_value = self.evaluate(q, possible_world);
                return p_value || q_value;
            }

            Formula::Imply(box p, box q, _) =>
            {
                let p_value = self.evaluate(p, possible_world);
                let q_value = self.evaluate(q, possible_world);
                return !p_value || q_value;
            }

            Formula::BiImply(box p, box q, _) =>
            {
                let p_value = self.evaluate(p, possible_world);
                let q_value = self.evaluate(q, possible_world);
                return p_value == q_value;
            }

            Formula::Possible(box p, _) =>
            {
                let accessible_worlds = self.vertices.iter()
                    .filter(|vertex| vertex.from == possible_world)
                    .map(|vertex| vertex.to)
                    .collect::<BTreeSet<PossibleWorld>>();
                if accessible_worlds.is_empty() { return false }

                return self.nodes.iter()
                    .filter(|node| accessible_worlds.contains(&node.possible_world))
                    .any(|node| self.evaluate(p, node.possible_world));
            }

            Formula::Necessary(box p, _) =>
            {
                let accessible_worlds = self.vertices.iter()
                    .filter(|vertex| vertex.from == possible_world)
                    .map(|vertex| vertex.to)
                    .collect::<BTreeSet<PossibleWorld>>();
                if accessible_worlds.is_empty() { return false }

                return self.nodes.iter()
                    .filter(|node| accessible_worlds.contains(&node.possible_world))
                    .all(|node| self.evaluate(p, node.possible_world));
            }

            _ => { false }
        }
    }
}
