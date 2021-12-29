use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum DataKind {
    // todo: more kind.
    Vertex,
    Edge,
    Path,
    Boolean,
    Integer,
    String,
    Null,
}

impl Default for DataKind {
    fn default() -> Self { 
        DataKind::Null 
    }
}

#[derive(Debug)]
pub struct VariableManager {
    types: HashMap<DataKind, Vec<String>>,
}

impl Default for VariableManager {
    fn default() -> Self {
        VariableManager {
            types: HashMap::new(),
        }
    }
}

impl VariableManager {
    pub fn add_variable(&mut self, var: String, kind: DataKind) {
        let vars = self.types.entry(kind).or_insert_with(Vec::new);
        vars.push(var);
    }

    /// return a variable of the taeget type randomly.
    /// todo: delete unwrap and add random select.
    pub fn random_variable(&mut self, target: DataKind) -> String {
        let vars = self.types.get(&target).unwrap();
        let idx = 0usize;
        vars.get(idx).unwrap().clone()
    }
}