use std::collections::HashMap;

use super::{DataType, Literal};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum DataKind {
    // todo: more kind.
    Vertex,
    Edge,
    Path,
    Boolean,
    Numerical,
    List,
    Map,
    String,
    Pipe,
    Null,
    Time,
    // TODO: function to type map.
    Function,
    // TODO: cypher query to type map.
    Query,
}

impl Default for DataKind {
    fn default() -> Self {
        DataKind::Null
    }
}

impl From<DataType> for DataKind {
    fn from(kind: DataType) -> Self {
        match kind {
            DataType::Null => DataKind::Null,
            DataType::Bool => DataKind::Boolean,
            DataType::Int32 | DataType::Int64 | DataType::Double | DataType::Float => {
                DataKind::Numerical
            }
            DataType::Date | DataType::Datetime => DataKind::Time,
            DataType::String => DataKind::String,
        }
    }
}

impl From<Literal> for DataKind {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Double(_) | Literal::Integer(_) => DataKind::Numerical,
            Literal::String(_) => DataKind::String,
            Literal::Boolean(_) => DataKind::Boolean,
            Literal::List(_) => DataKind::List,
            Literal::Map(_) => DataKind::Map,
            Literal::Null => DataKind::Null,
        }
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
