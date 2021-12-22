use super::Label;

pub struct 

GraphSchema {
    // graph name
    name: String,
    // node labels
    vertex_labels: Vec<Label>,
    // relationship labels
    edge_labels: Vec<Label>,
}

impl GraphSchema {
    fn new(name: String, labels: Vec<Label>) -> GraphSchema {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}