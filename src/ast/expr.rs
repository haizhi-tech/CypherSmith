pub struct Variable {
    variable_name: String,
}

impl Variable {
    pub fn new() -> Self {
        Variable {
            variable_name: "a".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.variable_name.clone()
    }
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
