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

pub struct ExpressionGenerator {}

#[derive(Debug)]
pub enum ExprKind {
    /// A binary operator expression (e.g., `a+2`).
    BinOp(BinOpKind),
    /// A unary operator expression (e.g., `-x`).
    UnOp(UnOpKind),
    /// A comparison chain (e.g. `a+b>1+c=d`).
    /// Invariant: v.len() > 0.
    Cmp(CmpKind),
    /// A literal.
    Lit(Literal),
    /// A Variable,
    Variable(String),
    /// A predicate variable,
    PredicateVariable(String),
    /// A case expression (e.g. ...),
    Case,
    /// A property access (e.g. `a.age`),
    Property,
    /// A function invocation (e.g. `sin(a)`),
    Invocation,
    /// A predicate function,
    PredicateFunction,
}

#[derive(Debug)]
pub enum BinOpKind {
    /// The `OR` operator (logical or).
    Or,
    /// The `XOR` operator (logical xor).
    Xor,
    /// The `AND` operator (logical and).
    And,
    /// The `+` operator (addition).
    Add,
    /// The `-` operator (subtraction).
    Sub,
    /// The `*` operator (multiplication).
    Mul,
    /// The `/` operator (division).
    Div,
    /// The `%` operator (modulo).
    Mod,
    /// The `^` operator (power).
    Pow,
    /// The `[_]` operator (index).
    Index,
    /// The `IN` operator.
    In,
    /// The `CONTAINS` operator
    Contains,
    /// The `STARTS WITH` operator
    StartsWith,
    /// The `ENDS WITH` operator
    EndsWith,
}

#[derive(Debug)]
pub enum UnOpKind {
    /// The `+` operator (positive).
    Pos,
    /// The `-` operator (negative).
    Neg,
    /// The `NOT` operator (logical not).
    Not,
}

#[derive(Debug)]
pub enum CmpKind {
    /// The `<>` operator.
    Ne,
    /// The `=` operator.
    Eq,
    /// The `<` operator.
    Lt,
    /// The `>` operator.
    Gt,
    /// The `<=` operator.
    Le,
    /// The `>=` operator.
    Ge,
}

#[derive(Debug)]
pub enum Literal {
    Double(f64),
    Integer(u64),
    String(String),
    Boolean(bool),
    // List(Vec<Box<ExpressionNode>>),
    // Map(Vec<(String, Box<ExpressionNode>)>),
    Null,
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
