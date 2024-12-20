use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;
use std::rc::Rc;
use num_bigint::BigUint;
use num_traits::One;
use rand::prelude::SliceRandom;
use crate::countermodel::{CountermodelGraph, CountermodelGraphNode, CountermodelGraphVertex};
use crate::formula::PossibleWorld;
use crate::logic::Logic;
use crate::utils::{get_config_value, CONFIG_KEY_SHOULD_SHUFFLE_COUNTERMODEL_GRAPHS};

pub struct CountermodelGraphGenerator
{
    pub logic : Rc<dyn Logic>,
    pub atomic_names : BTreeSet<String>,
}

impl CountermodelGraphGenerator
{
    fn generate_graph_codes(&self, number_of_nodes : u8) -> Vec<BigUint>
    {
        let mut random = rand::thread_rng();
        let mut codes : Vec<BigUint> = Vec::new();
        let mut code = BigUint::ZERO;

        while code < BigUint::from(2u8).pow(number_of_nodes.pow(2) as u32)
        {
            codes.push(code.clone());
            code = code.add(BigUint::one());
        }

        if get_config_value::<bool>(CONFIG_KEY_SHOULD_SHUFFLE_COUNTERMODEL_GRAPHS).unwrap_or_default()
        {
            codes.shuffle(&mut random);
        }

        return codes;
    }

    pub fn generate_graphs(&self, number_of_nodes : u8) -> Vec<CountermodelGraph>
    {
        let mut generated_graphs : Vec<CountermodelGraph> = vec![];

        for code in self.generate_graph_codes(number_of_nodes)
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

            if graph.validate(&self.logic).is_ok()
            {
                generated_graphs.push(graph);
            }
        }

        return generated_graphs;
    }
}
