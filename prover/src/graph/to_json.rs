use serde::{Deserialize, Serialize};
use crate::graph::Graph;

#[derive(Serialize, Deserialize)]
pub struct GraphJSON
{
    nodes : Vec<String>,
    vertices : Vec<GraphVertexJSON>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphVertexJSON
{
    from : String, to : String,
    tags : Vec<String>,
}

impl Graph
{
    pub fn to_json(&self) -> GraphJSON
    {
        let nodes_strings = self.nodes.iter().map(|n| n.to_string()).collect();

        let mut vertices_json: Vec<GraphVertexJSON> = vec![];
        for vertex in &self.vertices
        {
            let tags = self.vertices_tags.iter()
                .filter(|(v, _tag)| v.from==vertex.from && v.to==vertex.to)
                .map(|(_v, tag)| tag.clone()).collect::<Vec<String>>();

            vertices_json.push(GraphVertexJSON
            {
                from: vertex.from.to_string(),
                to: vertex.to.to_string(),
                tags: tags,
            })
        }

        return GraphJSON { nodes:nodes_strings, vertices:vertices_json };
    }
}
