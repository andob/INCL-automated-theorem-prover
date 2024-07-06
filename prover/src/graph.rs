pub mod to_json;
mod missing_vertices;

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::formula::PossibleWorld;
use crate::logic::common_modal_logic::NecessityReapplicationData;

pub struct Graph
{
    pub nodes : HashSet<PossibleWorld>,
    pub vertices : HashSet<GraphVertex>,
    pub necessity_reapplications : Vec<NecessityReapplicationData>,
    log : String,
}

impl Graph
{
    pub fn new() -> Graph
    {
        return Graph
        {
            nodes: HashSet::new(),
            vertices: HashSet::new(),
            necessity_reapplications: vec![],
            log: String::new(),
        };
    }

    pub fn is_empty(&self) -> bool
    {
        return self.nodes.is_empty() && self.vertices.is_empty();
    }

    pub fn add_node(&mut self, node : PossibleWorld)
    {
        self.nodes.insert(node.clone());
    }

    pub fn add_vertex(&mut self, from : PossibleWorld, to : PossibleWorld)
    {
        self.log.push_str(format!("{}r{}\n", from, to).as_str());
        self.vertices.insert(GraphVertex::new(from, to));
    }

    pub fn flush_log(&mut self) -> String
    {
        let log = self.log.trim().to_string();
        self.log = String::new();
        return log;
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct GraphVertex
{
    pub from : PossibleWorld,
    pub to : PossibleWorld,
}

impl GraphVertex
{
    pub fn new(from : PossibleWorld, to : PossibleWorld) -> GraphVertex
    {
        return GraphVertex { from, to };
    }
}

impl Display for GraphVertex
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{} -> {}", self.from, self.to);
    }
}
