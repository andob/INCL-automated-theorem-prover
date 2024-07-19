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
    log_line_formatter : fn(&GraphVertex) -> String,
    log : String,
}

#[macro_export]
macro_rules! default_log_line_formatter
{
    () => { |vertex| format!("{}R{}\n", vertex.from, vertex.to) };
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
            log_line_formatter: default_log_line_formatter!(),
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
        let vertex = GraphVertex::new(from, to);
        self.log_vertex(&vertex);
        self.vertices.insert(vertex);
    }

    pub fn set_log_line_formatter(&mut self, formatter : fn(&GraphVertex) -> String)
    {
        self.log_line_formatter = formatter;
    }

    pub fn log_vertex(&mut self, vertex : &GraphVertex)
    {
        self.log.push_str((self.log_line_formatter)(&vertex).as_str());
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
