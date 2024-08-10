use std::collections::BTreeSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use substring::Substring;
use crate::formula::PossibleWorld;
use crate::graph::Graph;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct GraphJSON
{
    nodes : BTreeSet<PossibleWorld>,
    vertices : BTreeSet<GraphVertexJSON>,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct GraphVertexJSON
{
    from : PossibleWorld,
    to : PossibleWorld,
    tags : Vec<String>,
}

impl Serialize for PossibleWorld
{
    fn serialize<S>(&self, serializer : S) -> Result<S::Ok, S::Error> where S : Serializer
    {
        return serializer.serialize_str(self.to_string().as_str());
    }
}

impl <'de> Deserialize<'de> for PossibleWorld
{
    fn deserialize<D>(deserializer : D) -> Result<Self, D::Error> where D : Deserializer<'de>
    {
        let string = String::deserialize(deserializer)?;
        let index = string.substring(1, string.len()-1).parse::<u8>().unwrap_or_default();
        return Ok(PossibleWorld { index });
    }
}

impl Graph
{
    pub fn to_json(&self) -> GraphJSON
    {
        let mut vertices_json : BTreeSet<GraphVertexJSON> = BTreeSet::new();
        for vertex in &self.vertices
        {
            let tags = self.vertices_tags.iter()
                .filter(|(v, _tag)| v.from==vertex.from && v.to==vertex.to)
                .map(|(_v, tag)| tag.clone()).collect::<Vec<String>>();

            vertices_json.insert(GraphVertexJSON { from:vertex.from, to:vertex.to, tags:tags });
        }

        return GraphJSON { nodes:self.nodes.clone(), vertices:vertices_json }
    }
}
