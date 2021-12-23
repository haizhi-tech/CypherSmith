use serde::{Deserialize, Serialize};

use crate::common::{LabelId, Property, RandomGenerator};

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
    pub label_name: String,
    pub label_id: LabelId,
    pub kind: LabelKind,
    pub properties: Vec<Property>,
}

impl Label {
    pub fn is_vertex(&self) -> bool {
        self.kind.is_vertex()
    }

    // return the label name
    pub fn get_name(&self) -> String {
        self.label_name.clone()
    }

    /// get random property
    pub fn random_property(&self, random: &mut RandomGenerator) -> Property {
        let idx = random.under(self.properties.len() as _);
        self.properties[idx as usize].clone()
    }

    /// get random properties without repeat.
    pub fn random_properties(&self, number: i32, random: &mut RandomGenerator) -> Vec<Property> {
        if number <= 0 {
            return vec![];
        }
        if number as usize > self.properties.len() {
            return self.properties.clone();
        }
        let length = self.properties.len() as i32;

        (0..number)
            .into_iter()
            .map(|_| {
                let idx = random.under(length);
                self.properties[idx as usize].clone()
            })
            .collect::<Vec<Property>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
