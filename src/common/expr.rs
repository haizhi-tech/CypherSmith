use super::util::RESERVED_WORD;
use super::{DataKind, Property, RandomGenerator, VariableManager};
use crate::ast::CypherNode;

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
    name: String,
    number: u32,
    pub manager: VariableManager,
}

impl VariableGenerator {
    pub fn new() -> Self {
        VariableGenerator {
            name: "v".to_string(),
            number: 0u32,
            manager: VariableManager::default(),
        }
        // Variable::
    }

    pub fn current_variable(&self) -> String {
        self.name.clone() + &self.number.to_string()
    }

    pub fn new_variable(&mut self) -> Variable {
        self.number += 1u32;
        Variable::new(self.name.clone() + &self.number.to_string())
    }

    pub fn get_old_variable(&mut self) -> Variable {
        let mut random = RandomGenerator::new();
        let old_number = random.d100() % (self.number as i32);
        Variable::new(self.name.clone() + &old_number.to_string())
    }

    pub fn get_procedure_method(&mut self) -> Variable {
        Variable::new("shortestPath".to_string())
    }

    pub fn get_procedure_result(&mut self) -> Variable {
        Variable::new("procedure_result(WIP)".to_string())
    }

    pub fn get_symbolic_or_integer(&mut self) -> Variable {
        Variable::new("symbolic_or_integer(WIP)".to_string())
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

#[derive(Debug, Default)]
pub struct Expression {
    name: String,
    kind: DataKind,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            name: "expression(WIP)".to_string(),
            kind: DataKind::default(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl From<String> for Expression {
    fn from(s: String) -> Self {
        Expression {
            name: s,
            kind: DataKind::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PropertyExpression {
    name: String,
}

// todo: need to implementation.
impl PropertyExpression {
    pub fn new() -> Self {
        PropertyExpression {
            name: "a.age".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Default)]
pub struct NodeLabel {
    label_name: String,
}

// todo: need to implementation get old nodelabel.
impl NodeLabel {
    pub fn new() -> Self {
        NodeLabel {
            label_name: "NodeLabel(WIP)".to_string(),
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
    // todo: need to modify, SchemaName not correct.
    pub fn new(random: &mut RandomGenerator) -> Self {
        let label_name = if random.d12() < 6 {
            // Variable name
            NodeLabel::new().get_name()
        } else {
            // label_name == ReserverdWord
            let index = random.under(48);
            RESERVED_WORD[index as usize].to_string()
        };
        SchemaName { label_name }
    }

    pub fn get_name(&self) -> String {
        self.label_name.clone()
    }
}

#[derive(Debug)]
pub struct Expr {
    kind: ExprKind,
}

impl From<ExprKind> for Expr {
    fn from(kind: ExprKind) -> Self {
        Expr { kind }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    /// A binary operator expression (e.g., `a+2`).
    BinOp(BinOpKind, Box<Expr>, Box<Expr>),
    /// A unary operator expression (e.g., `-x`).
    UnOp(UnOpKind, Box<Expr>),
    /// A comparison chain (e.g. `a+b>1+c=d`).
    Cmp(Box<Expr>, Vec<(CmpKind, Box<Expr>)>),
    /// A literal.
    Lit(Literal),
    /// A Variable,
    Variable(Variable),
    /// A predicate variable,
    PredicateVariable(String),
    /// A case expression (e.g. ...),
    Case(Option<Box<Expr>>, Vec<CaseAlternative>, Option<Box<Expr>>),
    /// A property access (e.g. `a.age`),
    Property(Box<Expr>, Property),
    /// A function invocation (e.g. `sin(a)`),
    Invocation(Box<Expr>, bool, Vec<Expr>),
    /// A predicate function,
    PredicateFunction(PredicateFunctionKind, Variable, Box<Expr>, Box<Expr>),
    /// A apoc expression,
    ApocExpression(String, Vec<Expr>),
    /// A Subquery expression,
    SubQuery(SubQueryKind, Box<CypherNode>),
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum UnOpKind {
    /// The `+` operator (positive).
    Pos,
    /// The `-` operator (negative).
    Neg,
    /// The `NOT` operator (logical not).
    Not,
    /// The `IS NULL` operator.
    Null,
    /// The `IS NOT NULL` operator.
    NotNull,
    //// The `()` operator.
    Parentheses,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum PredicateFunctionKind {
    /// The `ALL` function.
    All,
    /// The `ANY` function.
    Any,
    /// The `NONE` function.
    None,
    /// The `SINGLE` function.
    Single,
}

#[derive(Debug, Copy, Clone)]
pub enum SubQueryKind {
    /// The `Exists { Query }` function
    Exists,
}

/// Case Alternative.
///
/// # Synopsis
/// > **WHEN** *Expression* **THEN** *Expression*
#[derive(Debug)]
pub struct CaseAlternative {
    pub condition: Box<Expr>,
    pub value: Box<Expr>,
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

#[derive(Debug)]
pub enum Literal {
    Double(f64),
    Integer(u64),
    String(String),
    Boolean(bool),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    Null,
}
