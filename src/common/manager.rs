use std::collections::HashMap;

#[derive(Debug)]
pub enum VariableKind {
    // todo: more kind need add in.
    Vertex,
    Edge,
    Path,
    Boolean,
    Integer,
    String,
}

#[derive(Debug)]
pub struct VariableManager {
    types: HashMap<String, VariableKind>,
}

impl Default for VariableManager {
    fn default() -> Self {
        VariableManager {
            types: HashMap::new(),
        }
    }
}

impl VariableManager {
    pub fn add_type(&mut self, var: String, kind: VariableKind) {
        self.types.insert(var, kind);
    }
}