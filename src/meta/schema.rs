use serde::{Deserialize, Serialize};

use super::Label;
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
    fn test() {

    }
}