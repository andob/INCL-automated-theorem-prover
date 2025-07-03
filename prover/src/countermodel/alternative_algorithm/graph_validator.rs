use std::rc::Rc;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use crate::countermodel::CountermodelGraph;
use crate::logic::Logic;
use crate::logic::normal_modal_logic::NormalModalLogic;

impl CountermodelGraph
{
    pub fn validate(&self, logic : &Rc<dyn Logic>) -> Result<()>
    {
        let mut validation_message = String::new();
        let mut is_valid = true;

        if self.nodes.is_empty()
        {
            validation_message.push_str("Invalid graph: no nodes!");
            is_valid = false;
        }

        if self.vertices.is_empty()
        {
            validation_message.push_str("Invalid graph: no vertices!");
            is_valid = false;
        }

        if !self.nodes.iter()
            .filter(|node| node.possible_world.index>0)
            .all(|node| self.vertices.iter().any(|vertex|
                vertex.from != vertex.to && vertex.to == node.possible_world))
        {
            validation_message.push_str("Invalid graph: completely disconnected worlds!");
            is_valid = false;
        }

        if logic.get_name().is_normal_modal_logic()
        {
            let logic = logic.cast_to::<NormalModalLogic>().unwrap();
            if logic.is_reflexive && !self.is_reflexive()
            {
                validation_message.push_str("Invalid graph: not reflexive!");
                is_valid = false;
            }

            if logic.is_symmetric && !self.is_symmetric()
            {
                validation_message.push_str("Invalid graph: not symmetric!");
                is_valid = false;
            }

            if logic.is_transitive && self.is_transitive()
            {
                validation_message.push_str("Invalid graph: not transitive!");
                is_valid = false;
            }
        }

        return if is_valid { Ok(()) } else { Err(anyhow!(validation_message)) };
    }

    pub fn is_reflexive(&self) -> bool
    {
        return self.nodes.iter().all(|node|
            self.vertices.iter().any(|vertex|
                vertex.from == node.possible_world && vertex.from == vertex.to))
    }

    pub fn is_symmetric(&self) -> bool
    {
        return self.vertices.iter()
            .filter(|v1| v1.from != v1.to)
            .all(|v1| self.vertices.iter()
                .filter(|v2| v2.from != v2.to)
                .any(|v2| v1.from == v2.to && v1.to == v2.from))
    }

    pub fn is_transitive(&self) -> bool
    {
        return !self.vertices.iter().cartesian_product(self.vertices.iter())
            .filter(|(v1, v2)| v1.from != v1.to && v2.from != v2.to && v2.from == v1.to)
            .all(|(v1, v2)| self.vertices.iter()
                .any(|v3| v3.from == v1.from && v3.to == v2.to))
    }
}
