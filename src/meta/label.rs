use serde::{Deserialize, Serialize};

use crate::common::{LabelId, Property};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelKind {
    Vertex,
    Edge {
        /// Relations paris
        /// First string is src label name
        /// Second string is dst label name
        relations: Vec<(LabelId, LabelId)>,
        /// Directed flag.
        /// True means this edge label has direction, otherwise not.
        is_directed: bool,
    },
}

impl LabelKind {
    pub fn is_vertex(&self) -> bool {
        match self {
            LabelKind::Vertex => true,
            LabelKind::Edge {
                relations: _,
                is_directed: _,
            } => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    label_name: String,
    label_id: LabelId,
    kind: LabelKind,
    properties: Vec<Property>,
}

impl Label {
    pub fn is_vertex(&self) -> bool {
        self.kind.is_vertex()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}