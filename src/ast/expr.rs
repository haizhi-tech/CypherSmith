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
}

// resverserd words number: 45
pub enum ReservedWord {
    All,
    And,
    As,
    Asc,
    Ascending,
    By,
    Case,
    Create,
    Delete,
    Desc,
    Descending,
    Detach,
    Distinct,
    Drop,
    Else,
    End,
    Ends,
    Exists,
    False,
    In,
    Is,
    Limit,
    Match,
    Merge,
    Not,
    Null,
    On,
    Optional,
    Or,
    Order,
    Remove,
    Return,
    Set,
    Skip,
    Starts,
    Then,
    To,
    True,
    Union,
    Unique,
    Unwind,
    When,
    Where,
    With,
    Xor,
}

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

pub struct ReserverdWord {}

pub struct IntegerLiteral {}

pub struct Paramter {}

#[derive(Debug, Default)]
pub struct Expression {
    expression_name: String,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            expression_name: "a".to_string(),
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

impl NodeLabel {
    pub fn new() -> Self {
        NodeLabel {
            label_name: "Person".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VariableGenerator;

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
}
