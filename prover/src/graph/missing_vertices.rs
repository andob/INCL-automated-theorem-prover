use box_macro::bx;
use crate::default_log_line_formatter;
use crate::graph::{Graph, GraphVertex};

impl Graph
{
    pub fn add_missing_reflexive_vertices(&mut self)
    {
        let mut vertices_to_add : Vec<GraphVertex> = vec![];

        for node in &self.nodes
        {
            let reflexive_vertex = GraphVertex::new(*node, *node);
            if !self.vertices.contains(&reflexive_vertex)
            {
                vertices_to_add.push(reflexive_vertex);
            }
        }

        self.set_log_line_formatter(bx!(|v| format!("{}ρ{}\n", v.from, v.to)));
        self.add_vertices(vertices_to_add);

        self.log_line_formatter = default_log_line_formatter!();
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

        self.set_log_line_formatter(bx!(|v| format!("{}σ{}\n", v.from, v.to)));
        self.add_vertices(vertices_to_add);

        self.log_line_formatter = default_log_line_formatter!();
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

        self.set_log_line_formatter(bx!(|v| format!("{}τ{}\n", v.from, v.to)));
        self.add_vertices(vertices_to_add);

        self.log_line_formatter = default_log_line_formatter!();
    }
}
