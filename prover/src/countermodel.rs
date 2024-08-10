use crate::formula::Formula::{Atomic, Necessary, Non, StrictImply};
use crate::formula::{PossibleWorld, Sign};
use crate::graph::GraphVertex;
use crate::tree::path::ProofTreePath;
use crate::tree::ProofTree;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize, Deserialize)]
pub struct CountermodelGraph
{
    pub nodes : BTreeSet<CountermodelGraphNode>,
    pub vertices : BTreeSet<CountermodelGraphVertex>,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct CountermodelGraphNode
{
    pub possible_world : PossibleWorld,
    pub is_normal_world : bool,
    pub atomics : BTreeMap<String, Option<bool>>,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct CountermodelGraphVertex
{
    pub from : PossibleWorld,
    pub to : PossibleWorld,
    pub tags : Vec<String>,
}

impl ProofTree
{
    pub fn find_countermodel(&self) -> Option<CountermodelGraph>
    {
        if self.has_timeout || self.is_proof_correct
        {
            return None;
        }

        let logic = self.problem.logic.clone();
        if logic.get_name().is_first_order_logic()
        {
            //not yet implemented on First Order Logic
            return None;
        }

        let atomic_names = self.problem.premises.iter()
            .chain(Some(&self.problem.conclusion).into_iter())
            .flat_map(|formula| formula.get_all_atomic_names())
            .collect::<BTreeSet<String>>();

        let path = self.get_all_paths().into_iter()
            .find(|path| !path.is_contradictory(&logic))?;

        let mut graph_nodes : BTreeSet<CountermodelGraphNode> = BTreeSet::new();
        let mut graph_vertices : BTreeSet<CountermodelGraphVertex> = BTreeSet::new();

        let possible_worlds = path.nodes.iter()
            .map(|node| node.formula.get_possible_world())
            .collect::<BTreeSet<PossibleWorld>>();

        for possible_world in possible_worlds
        {
            graph_nodes.insert(CountermodelGraphNode
            {
                possible_world: possible_world,
                is_normal_world: self.check_if_possible_world_is_normal(possible_world, &path),
                atomics: self.populate_atomics(&atomic_names, &path, possible_world),
            });

            self.populate_with_graph_vertices(possible_world, &mut graph_vertices);
        }

        return Some(CountermodelGraph { nodes:graph_nodes, vertices:graph_vertices });
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

    fn populate_with_graph_vertices(&self, possible_world : PossibleWorld, output_vertices : &mut BTreeSet<CountermodelGraphVertex>)
    {
        let original_vertices = self.modality_graph.vertices.iter()
            .filter(|vertex| vertex.from == possible_world || vertex.to == possible_world)
            .collect::<BTreeSet<&GraphVertex>>();

        for original_vertex in original_vertices
        {
            let tags = self.modality_graph.vertices_tags.iter()
                .filter(|(vertex, _tag)| vertex == original_vertex)
                .map(|(_vertex, tag)| tag.clone())
                .collect::<Vec<String>>();

            output_vertices.insert(CountermodelGraphVertex
            {
                from: original_vertex.from,
                to: original_vertex.to, tags,
            });
        }
    }

    fn populate_atomics(&self, atomic_names : &BTreeSet<String>, path : &ProofTreePath, possible_world : PossibleWorld) -> BTreeMap<String, Option<bool>>
    {
        let mut values : BTreeMap<String, Option<bool>> = BTreeMap::new();

        if self.problem.logic.get_semantics().number_of_truth_values() == 2
        {
            for p in atomic_names.iter()
            {
                let found_p_on_path = path.nodes.iter()
                    .filter(|node| node.formula.get_possible_world() == possible_world)
                    .filter_map(|node| if let Atomic(p, _) = &node.formula { Some(p) } else { None })
                    .any(|q| p == q);

                if found_p_on_path
                {
                    values.insert(p.clone(), Some(true));
                }
                else
                {
                    values.insert(p.clone(), Some(false));
                }
            }
        }
        else
        {
            for p in atomic_names.iter()
            {
                let found_p_on_path = path.nodes.iter()
                    .filter(|node| node.formula.get_possible_world() == possible_world)
                    .filter(|node| node.formula.get_sign() == Sign::Plus)
                    .filter_map(|node| if let Atomic(p, _) = &node.formula { Some(p) } else { None })
                    .any(|p_on_path| p_on_path == p);

                let found_non_p_on_path = path.nodes.iter()
                    .filter(|node| node.formula.get_possible_world() == possible_world)
                    .filter(|node| node.formula.get_sign() == Sign::Plus)
                    .filter_map(|node| if let Non(box Atomic(p, _), _) = &node.formula { Some(p) } else { None })
                    .any(|q| p == q);

                if found_p_on_path
                {
                    values.insert(p.clone(), Some(true));
                }
                else if found_non_p_on_path
                {
                    values.insert(p.clone(), Some(false));
                }
                else
                {
                    values.insert(p.clone(), None);
                }
            }
        }

        return values;
    }
}
