use serde::{Deserialize, Serialize};
use crate::graph::{Graph, GraphVertex};

#[derive(Serialize, Deserialize)]
pub struct GraphJSON
{
    nodes : Vec<String>,
    vertices : Vec<GraphVertexJSON>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphVertexJSON
{
    from : String,
    to : String,
}

impl Graph
{
    pub fn to_json(&self) -> GraphJSON
    {
        return GraphJSON
        {
            nodes: self.nodes.iter().map(|node| node.to_string()).collect(),
            vertices: self.vertices.iter().map(|vertex| vertex.to_json()).collect(),
        }
    }
}

impl GraphVertex
{
    pub fn to_json(&self) -> GraphVertexJSON
    {
        return GraphVertexJSON
        {
            from: self.from.to_string(),
            to: self.to.to_string(),
        }
    }
}
