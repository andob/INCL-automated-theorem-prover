use crate::graph::{Graph, GraphVertex};

impl Graph
{
    pub fn add_missing_reflexive_vertices(&mut self)
    {
        for node in &self.nodes
        {
            let reflexive_vertex = GraphVertex::new(*node, *node);
            if !self.vertices.contains(&reflexive_vertex)
            {
                self.vertices.insert(reflexive_vertex);
            }
        }
    }

    pub fn add_missing_symmetric_vertices(&mut self)
    {
        let mut vertices_to_add : Vec<GraphVertex> = vec![];

        for vertex in &self.vertices
        {
            let symmetric_vertex = GraphVertex::new(vertex.to, vertex.from);
            if !self.vertices.contains(&symmetric_vertex)
            {
                vertices_to_add.push(symmetric_vertex);
            }
        }

        for vertex in vertices_to_add
        {
            self.vertices.insert(vertex);
        }
    }

    pub fn add_missing_transitive_vertices(&mut self)
    {
        let mut vertices_to_add : Vec<GraphVertex> = vec![];

        for i_vertex in &self.vertices
        {
            for j_vertex in &self.vertices
            {
                if i_vertex != j_vertex && i_vertex.to == j_vertex.from
                {
                    let transitive_vertex = GraphVertex::new(i_vertex.from, j_vertex.to);
                    if !self.vertices.contains(&transitive_vertex)
                    {
                        vertices_to_add.push(transitive_vertex);
                    }
                }
            }
        }

        for vertex in vertices_to_add
        {
            self.vertices.insert(vertex);
        }
    }
}
