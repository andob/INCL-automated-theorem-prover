use std::collections::{BTreeMap, BTreeSet};
use serde::{Deserialize, Serialize};
use crate::formula::PossibleWorld;

mod main_algorithm;
mod alternative_algorithm;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CountermodelGraph
{
    pub nodes : BTreeSet<CountermodelGraphNode>,
    pub vertices : BTreeSet<CountermodelGraphVertex>,
}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CountermodelGraphNode
{
    pub possible_world : PossibleWorld,
    pub is_normal_world : bool,
    pub atomics : BTreeMap<String, bool>,
}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CountermodelGraphVertex
{
    pub from : PossibleWorld,
    pub to : PossibleWorld,
    pub tags : Vec<String>,
}

impl CountermodelGraph
{
    pub fn new() -> CountermodelGraph
    {
        return CountermodelGraph
        {
            nodes: BTreeSet::new(),
            vertices: BTreeSet::new(),
        };
    }
}
