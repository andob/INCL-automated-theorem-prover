pub mod to_json;
mod missing_vertices;

use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use crate::formula::{Formula, PossibleWorld};
use crate::logic::common_modal_logic::NecessityReapplicationData;
use crate::proof::execution_log::ExecutionLogHelperData;

//todo refactor: make all fields private
pub struct Graph
{
    pub nodes : BTreeSet<PossibleWorld>,
    pub vertices : BTreeSet<GraphVertex>,
    pub vertices_tags : Vec<(GraphVertex, Formula)>,
    pub necessity_reapplications : Vec<NecessityReapplicationData>,
    log_line_formatter : Box<dyn Fn(&GraphVertex) -> String>,
    log : String,
}

#[macro_export]
macro_rules! default_log_line_formatter
{
    () => { Box::new(|vertex| format!("{}R{}\n", vertex.from, vertex.to)) };
}

impl Graph
{
    pub fn new() -> Graph
    {
        return Graph
        {
            nodes: BTreeSet::new(),
            vertices: BTreeSet::new(),
            vertices_tags: vec![],
            necessity_reapplications: vec![],
            log_line_formatter: default_log_line_formatter!(),
            log: String::new(),
        };
    }

    pub fn is_empty(&self) -> bool
    {
        return self.nodes.is_empty() && self.vertices.is_empty();
    }

    pub fn add_and_log_node(&mut self, node : PossibleWorld)
    {
        self.nodes.insert(node);

        ExecutionLogHelperData::with(|mut helper_data|
            helper_data.new_graph_nodes.insert(node));
    }

    pub fn add_and_log_vertex(&mut self, vertex : GraphVertex)
    {
        self.log.push_str((self.log_line_formatter)(&vertex).as_str());

        self.vertices.insert(vertex.clone());

        ExecutionLogHelperData::with(|mut helper_data|
            helper_data.new_graph_vertices.insert(vertex));
    }

    pub fn add_and_log_vertices(&mut self, vertices_to_add : Vec<GraphVertex>)
    {
        for vertex in vertices_to_add
        {
            self.log.push_str((self.log_line_formatter)(&vertex).as_str());

            self.vertices.insert(vertex.clone());

            ExecutionLogHelperData::with(|mut helper_data|
                helper_data.new_graph_vertices.insert(vertex));
        }
    }

    pub fn set_log_line_formatter(&mut self, formatter : Box<dyn Fn(&GraphVertex) -> String>)
    {
        self.log_line_formatter = formatter;
    }

    pub fn flush_log(&mut self) -> String
    {
        let log = self.log.trim().to_string();
        self.log = String::new();
        return log;
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
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

impl Debug for GraphVertex
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        return write!(f, "{}→{}", self.from, self.to);
    }
}
