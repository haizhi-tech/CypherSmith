use paste::paste;

macro_rules! expression_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident {},
    )* ) => {
        paste! {
            pub trait ExpressionNodeVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self) -> Self::Output;
                )*

                fn visit(&mut self) -> Self::Output {
                    // self.visit_query()
                    todo!()
                }
            }
        }
    };

}

expression_nodes_impl! {

    /// Expression: OrExpression
    Expression {},

    /// OrExpression: Vec<XorExpression>
    OrExpression {},

    /// XorExpression: Vec<AndExpression>
    XorExpression {},

    /// AndExpression: Vec<NotExpression>
    AndExpression {},

    /// NotExpression: Not? ComparsionExpression
    NotExpression {},

    /// ComparsionExpression: AddOrSubtractExpression + Vec<PartialComparisonExpression>
    ComparisonExpression {},

    /// PartialComparisonExpression: = <> < > <= >= AddOrSubtractExpression
    PartialComparisonExpression {},

    /// AddOrSubtractExpression: MultiplyDivideModuloExpression (+/- MultiplyDivideModuloExpression)*
    AddOrSubtractExpression {},

    /// MultiplyDivideModuloExpression: PowerOfExpression (*///% PowerOfExpression)*
    MultiplyDivideModuloExpression {},

    /// PowerOfExpression: UnaryAddOrSubtractExpression (^ UnaryAddOrSubtractExpression)*
    PowerOfExpression {},

    /// UnaryAddOrSubtractExpression: (+/-)* StringListNullOperatorExpression
    UnaryAddOrSubtractExpression {},

    /// StringListNullOperatorExpression: PropertyOrLabelsExpression, (StringOperatorExpression|ListOperatorExpression|NullOperatorExpression)*
    StringListNullOperatorExpression {},

    /// PropertyOrLabelsExpression: Atom, (PropertyLookup)*, (NodeLabels)+
    PropertyOrLabelsExpression {},

    /// StringOperatorExpression: (STARTS WITH | ENDS WITH | CONTAINS)? PropertyOrLabelsExpression
    StringOperatorExpression {},

    /// ListOperatorExpression:
    ListOperatorExpression {},

    /// NullOperatorExpression: IS NULL/ IS NOT NULL.
    NullOperatorExpression {},
}

/// PropertyLookUp: ". PropertyKeyName"
/// Atom:
/// NodeLabels:
///

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

// pub ComparisonExpression
