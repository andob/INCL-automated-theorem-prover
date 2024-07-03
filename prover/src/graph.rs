pub mod to_json;
mod missing_vertices;

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::formula::PossibleWorld;

pub struct Graph
{
    pub nodes : HashSet<PossibleWorld>,
    pub vertices : HashSet<GraphVertex>,
    pub log : String,
}

impl Graph
{
    pub fn new() -> Graph
    {
        let mut graph = Graph
        {
            nodes: HashSet::new(),
            vertices: HashSet::new(),
            log: String::new(),
        };

        graph.add_node(PossibleWorld::zero());
        return graph;
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
        self.vertices.insert(GraphVertex::new(from, to));

        self.log.push_str(format!("{}R{}", from, to).as_str());
    }

    pub fn clear_log(&mut self)
    {
        self.log = String::new();
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
