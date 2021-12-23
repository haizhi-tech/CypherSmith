use serde::{Deserialize, Serialize};

use super::Label;
use crate::common::RandomGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSchema {
    // graph name
    name: String,
    // node labels
    vertex_labels: Vec<Label>,
    // relationship labels
    edge_labels: Vec<Label>,
}

impl GraphSchema {
    pub fn new(name: String, labels: Vec<Label>) -> GraphSchema {
        let mut vertex_labels = Vec::new();
        let mut edge_labels = Vec::new();
        for label in labels.iter() {
            if label.is_vertex() {
                vertex_labels.push(label.clone());
            } else {
                edge_labels.push(label.clone());
            }
        }
        GraphSchema {
            name,
            vertex_labels,
            edge_labels,
        }
    }
}

impl GraphSchema {
    // todo: add error handling.
    pub fn rand_vertex_label(&self, random: &mut RandomGenerator) -> Label {
        let length = self.vertex_labels.len();
        // todo: return error.
        if length == 0 {}
        let idx = random.under(length as _);
        self.vertex_labels[idx as usize].clone()
    }

    pub fn rand_edge_label(&self, random: &mut RandomGenerator) -> Label {
        let idx = random.under(self.edge_labels.len() as _);
        self.edge_labels[idx as usize].clone()
    }
}

impl Default for GraphSchema {
    fn default() -> Self {
        GraphSchema {
            name: "test".to_string(),
            vertex_labels: vec![],
            edge_labels: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
