use crate::common::{RandomGenerator, RESERVED_WORD};

// pub enum Variable {
//     UnescapedSymbolicName,
//     EscapedSymbolicName,
//     HexLetter,
//     Function,
// }

#[derive(Debug, Default)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

// one ast tree use one
#[derive(Debug, Default)]
pub struct VariableGenerator {
    variable_name: String,
    number: u32,
}

// todo: need to modify. Variable manage and check.
impl VariableGenerator {
    pub fn new() -> Self {
        VariableGenerator {
            variable_name: "a".to_string(),
            number: 0u32,
        }
        // Variable::
    }

    pub fn current_variable(&self) -> String {
        self.variable_name.clone() + &self.number.to_string()
    }

    pub fn new_variable(&mut self) -> Variable {
        self.number += 1u32;
        Variable::new(self.variable_name.clone() + &self.number.to_string())
    }

    pub fn get_old_variable(&mut self) -> Variable {
        Variable::new(self.variable_name.clone() + &self.number.to_string())
    }

    pub fn get_procedure_method(&mut self) -> Variable {
        Variable::new("shortestPath".to_string())
    }

    pub fn get_procedure_result(&mut self) -> Variable {
        Variable::new("procedure_result".to_string())
    }
}

#[derive(Debug, Default)]
pub struct Properties {
    property_name: String,
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            property_name: "property(WIP)".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.property_name.clone()
    }
}

#[derive(Debug, Default)]
pub struct NameSpace {
    name_space: String,
}

impl NameSpace {
    pub fn new() -> Self {
        NameSpace {
            name_space: "atlas".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name_space.clone()
    }
}

#[derive(Debug)]
pub enum RelationshipDirection {
    // <- [] -
    Left,
    // - [] ->
    Right,
    // <- [] ->
    Both,
    // - [] -
    None,
}

pub struct SymbolicName {}

pub struct IntegerLiteral {}

pub struct Paramter {}

#[derive(Debug, Default)]
pub struct Expression {
    expression_name: String,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            expression_name: "expression(WIP)".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.expression_name.clone()
    }
}

#[derive(Debug, Default)]
pub struct PropertyExpression {
    expression_name: String,
}

// todo: need to implementation.
impl PropertyExpression {
    pub fn new() -> Self {
        PropertyExpression {
            expression_name: "a.age".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.expression_name.clone()
    }
}

#[derive(Debug, Default)]
pub struct NodeLabel {
    label_name: String,
}

// todo: need to implementation get old nodelabel.
impl NodeLabel {
    pub fn new() -> Self {
        // let label_name = if random.d12() < 6 { // Variable name

        // } else { // label_name == ReserverdWord
        //     let x = ReservedWord::Delete;
        // };
        NodeLabel {
            label_name: "Person".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.label_name.clone()
    }
}

#[derive(Debug, Default)]
pub struct SchemaName {
    label_name: String,
}

impl SchemaName {
    pub fn new(random: &mut RandomGenerator) -> Self {
        let label_name = if random.d12() < 6 {
            // Variable name
            NodeLabel::new().get_name()
        } else {
            // label_name == ReserverdWord
            let index = random.d42();
            RESERVED_WORD[index as usize].to_string()
        };
        SchemaName { label_name }
    }
}

#[cfg(test)]
mod tests {
    use super::{RandomGenerator, SchemaName, VariableGenerator};

    #[test]
    fn test_variable_generator() {
        let mut var = VariableGenerator::new();
        println!("{:?}", var);
        for _ in 0..5 {
            let new_var = var.new_variable();
            println!("{:?}", new_var);
        }
        println!("{:?}", var);
    }

    #[test]
    fn test_schema_name() {
        let mut random_gen = RandomGenerator::new();
        let new_schema_name = SchemaName::new(&mut random_gen);
        println!("{:?}", new_schema_name);
    }
}
