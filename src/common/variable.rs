use std::{collections::HashMap, fmt::Display};

use super::{DataType, Diagnostic, Literal, RandomGenerator};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum DataKind {
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
            Literal::NullValue => DataKind::Null,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Variable {
    name: String,
    kind: DataKind,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable {
            name,
            kind: DataKind::default(),
        }
    }

    pub fn new_var(name: String, kind: DataKind) -> Self {
        Variable { name, kind }
    }
}

impl Variable {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_kind(&self) -> DataKind {
        self.kind.clone()
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Default, Debug)]
pub struct VariableManager {
    types: HashMap<DataKind, Vec<String>>,
    random: RandomGenerator,
}

impl VariableManager {
    pub fn add_variable(&mut self, var: String, kind: DataKind) {
        let vars = self.types.entry(kind).or_insert_with(Vec::new);
        vars.push(var);
    }

    /// return a variable of the taeget type randomly.
    pub fn random_target_variable(&mut self, target: DataKind) -> Result<Variable, Diagnostic> {
        let vars = self
            .types
            .get(&target)
            .ok_or_else(|| Diagnostic::warn("need retry", None))?;
        let idx = self.random.under(vars.len() as _);
        let var = vars
            .get(idx as usize)
            .ok_or_else(|| Diagnostic::bug("variable out of range.", None))?;
        Ok(Variable::new_var(var.to_string(), target))
    }
}

// one ast tree use one
#[derive(Debug, Default)]
pub struct VariableGenerator {
    name: String,
    number: u32,
    manager: VariableManager,
}

impl VariableGenerator {
    pub fn new() -> Self {
        VariableGenerator {
            name: "v".to_string(),
            number: 0u32,
            manager: VariableManager::default(),
        }
    }

    /// default variable.
    pub fn new_variable(&mut self) -> Variable {
        let var = Variable::new(self.name.clone() + &self.number.to_string());
        self.number += 1u32;
        var
    }

    /// variable with target kind.
    pub fn new_kind_variable(&mut self, kind: DataKind) -> Variable {
        let var_name = self.name.clone() + &self.number.to_string();
        let var = Variable::new_var(var_name.clone(), kind.clone());
        self.manager.add_variable(var_name, kind);
        self.number += 1u32;
        var
    }

    pub fn get_old_variable(&mut self) -> Variable {
        let mut random = RandomGenerator::new();
        let old_number = random.d100() % ((self.number + 1) as i32);
        Variable::new(self.name.clone() + &old_number.to_string())
    }

    /// get target datakind variable
    pub fn get_target_variable(&mut self, kind: DataKind) -> Result<Variable, Diagnostic> {
        self.manager.random_target_variable(kind)
    }

    /// procedure method.
    pub fn get_procedure_method(&mut self) -> Variable {
        Variable::new("shortestPath".to_string())
    }

    /// procedure result.
    pub fn get_procedure_result(&mut self) -> Variable {
        Variable::new("procedure_result(WIP)".to_string())
    }

    // pub fn get_symbolic_or_integer(&mut self) -> Variable {
    //     Variable::new("symbolic_or_integer(WIP)".to_string())
    // }
}
