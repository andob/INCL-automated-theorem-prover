use std::collections::{BTreeMap, BTreeSet};
use smol_str::SmolStr;
use crate::countermodel::{CountermodelGraph, CountermodelGraphNode, CountermodelGraphVertex};
use crate::formula::Formula::{Atomic, Necessary, StrictImply};
use crate::formula::PossibleWorld;
use crate::formula::to_string::FormulaFormatOptions;
use crate::graph::GraphVertex;
use crate::tree::path::ProofTreePath;
use crate::tree::ProofTree;

impl ProofTree
{
    pub fn find_countermodel(&self) -> Option<CountermodelGraph>
    {
        //no countermodel if proof is correct
        if self.is_proof_correct { return None };

        //on timeout, the alternate algorithm is used
        if self.has_timeout { return self.find_countermodel_alt() };

        //not yet implemented on first order logic and many valued logics
        let logic = self.problem.logic.clone();
        if logic.get_name().is_first_order_logic() { return None };
        if logic.get_semantics().number_of_truth_values() > 2 { return None };

        let atomic_names = self.problem.premises.iter()
            .chain(Some(&self.problem.conclusion).into_iter())
            .flat_map(|formula| formula.get_all_atomic_names())
            .collect::<BTreeSet<SmolStr>>();

        let path = self.get_all_paths().into_iter()
            .find(|path| !path.is_contradictory(&logic))?;

        let mut graph_nodes : BTreeSet<CountermodelGraphNode> = BTreeSet::new();
        let mut graph_vertices : BTreeSet<CountermodelGraphVertex> = BTreeSet::new();

        let possible_worlds = path.nodes.iter()
            .map(|node| node.formula.get_possible_world())
            .collect::<BTreeSet<PossibleWorld>>();

        for possible_world in &possible_worlds
        {
            graph_nodes.insert(CountermodelGraphNode
            {
                possible_world: *possible_world,
                is_normal_world: self.check_if_possible_world_is_normal(*possible_world, &path),
                atomics: self.populate_atomics(&atomic_names, &path, *possible_world),
            });
        }

        for possible_world in possible_worlds
        {
            self.populate_with_graph_vertices(possible_world, &graph_nodes, &mut graph_vertices);
        }

        return Some(CountermodelGraph
        {
            nodes: graph_nodes, vertices: graph_vertices,
            was_built_from_modality_graph: true,
            comment: String::new(),
        });
    }

    fn check_if_possible_world_is_normal(&self, possible_world : PossibleWorld, path : &ProofTreePath) -> bool
    {
        if self.problem.logic.get_name().is_non_normal_modal_logic()
        {
            return possible_world == PossibleWorld::zero() ||
                path.nodes.iter().any(|node|
                    node.formula.get_possible_world() == possible_world &&
                    matches!(node.formula, Necessary(..) | StrictImply(..)));
        }

        return true;
    }

    fn populate_with_graph_vertices(&self, possible_world : PossibleWorld,
        output_nodes : &BTreeSet<CountermodelGraphNode>, output_vertices : &mut BTreeSet<CountermodelGraphVertex>)
    {
        let formula_format_options = FormulaFormatOptions::default();

        let original_vertices = self.modality_graph.vertices()
            .filter(|vertex| (vertex.from == possible_world || vertex.to == possible_world) &&
                output_nodes.iter().any(|node| node.possible_world == vertex.from) &&
                output_nodes.iter().any(|node| node.possible_world == vertex.to))
            .collect::<BTreeSet<&GraphVertex>>();

        for original_vertex in original_vertices
        {
            let tags = self.modality_graph.vertices_tags()
                .filter(|(vertex, _tag)| vertex == original_vertex)
                .map(|(_v, tag)| tag.to_string_with_options(&formula_format_options))
                .collect::<Vec<String>>();

            output_vertices.insert(CountermodelGraphVertex
            {
                from: original_vertex.from,
                to: original_vertex.to, tags,
            });
        }
    }

    fn populate_atomics(&self, atomic_names : &BTreeSet<SmolStr>, path : &ProofTreePath, possible_world : PossibleWorld) -> BTreeMap<String, bool>
    {
        let mut values : BTreeMap<String, bool> = BTreeMap::new();

        for p in atomic_names.iter()
        {
            let p_value = path.nodes.iter()
                .filter(|node| node.formula.get_possible_world() == possible_world)
                .filter_map(|node| if let Atomic(p, _) = &node.formula { Some(p) } else { None })
                .any(|q| p == q);

            values.insert(p.to_string(), p_value);
        }

        return values;
    }
}
