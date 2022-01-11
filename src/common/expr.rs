use std::fmt::Display;

use super::{DataKind, Property, Variable};
use crate::{
    ast::{CypherNode, TransformVisitor},
    meta::Label,
};

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone)]
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
    PredicateVariable(Variable),
    /// A case expression (e.g. ...),
    Case(Option<Box<Expr>>, Vec<CaseAlternative>, Option<Box<Expr>>),
    /// A property access (e.g. `a.age`),
    Property(Box<Expr>, Property),
    /// Vertex Label.
    Label(Box<Expr>, Label),
    /// A function invocation (e.g. `sin(a)`),
    Invocation(Box<Expr>, bool, Vec<Expr>),
    /// FilterExpression: (e.g. a in [1,2] where a>1).
    FilterExpression(Variable, Box<Expr>, Option<Box<Expr>>),
    /// A predicate function,
    PredicateFunction(PredicateFunctionKind, Box<Expr>),
    /// A Subquery expression,
    SubQuery(SubQueryKind, Box<CypherNode>, Option<Box<Expr>>),
}

impl ExprKind {
    pub fn get_kind(&self) -> DataKind {
        match self {
            ExprKind::BinOp(kind, expr, _) => match kind {
                BinOpKind::Add
                | BinOpKind::Sub
                | BinOpKind::Mul
                | BinOpKind::Div
                | BinOpKind::Mod
                | BinOpKind::Pow => expr.kind.get_kind(),
                BinOpKind::Or
                | BinOpKind::Xor
                | BinOpKind::And
                | BinOpKind::Contains
                | BinOpKind::StartsWith
                | BinOpKind::EndsWith
                | BinOpKind::In => DataKind::Boolean,
                BinOpKind::Index => expr.kind.get_kind(),
                BinOpKind::Pipe => DataKind::Pipe,
            },
            ExprKind::UnOp(kind, expr) => match kind {
                UnOpKind::Pos | UnOpKind::Neg => expr.kind.get_kind(),
                UnOpKind::Not => DataKind::Boolean,
                UnOpKind::Null | UnOpKind::NotNull => DataKind::Boolean,
                UnOpKind::Parentheses => expr.kind.get_kind(),
            },
            ExprKind::Cmp(_, _) => DataKind::Boolean,
            ExprKind::Lit(literal) => DataKind::from(literal.clone()),
            ExprKind::Variable(var) => var.get_kind(),
            ExprKind::PredicateVariable(var) => var.get_kind(),
            ExprKind::Case(_, alternative, _) => alternative.iter().next().map_or_else(
                || DataKind::Null,
                |x| x.value.clone().as_ref().kind.get_kind(),
            ),
            ExprKind::Property(_, pro) => DataKind::from(pro.prop_type),
            ExprKind::Label(_, _) => DataKind::Vertex,
            ExprKind::Invocation(_, _, _) => DataKind::Function,
            ExprKind::PredicateFunction(_, _) => DataKind::Boolean,
            ExprKind::SubQuery(_, _, _) => DataKind::Query,
            ExprKind::FilterExpression(_, _, _) => DataKind::Boolean,
        }
    }
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
    /// The `|` Operator
    Pipe,
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

impl Display for CmpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CmpKind::Ne => f.write_str("<>"),
            CmpKind::Eq => f.write_str("="),
            CmpKind::Lt => f.write_str("<"),
            CmpKind::Gt => f.write_str(">"),
            CmpKind::Le => f.write_str("<="),
            CmpKind::Ge => f.write_str(">="),
        }
    }
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

impl Display for PredicateFunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PredicateFunctionKind::All => f.write_str("ALL"),
            PredicateFunctionKind::Any => f.write_str("ANY"),
            PredicateFunctionKind::None => f.write_str("NONE"),
            PredicateFunctionKind::Single => f.write_str("SINGLE"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SubQueryKind {
    /// The `Exists { Query }` function
    Exists,
    /// RelationShipsPattern:
    RelationShipsPattern,
    /// PredicatePattern: (Variable=)? RelationShipsPattern Where?
    PredicatePattern,
}

/// Case Alternative.
///
/// # Synopsis
/// > **WHEN** *Expression* **THEN** *Expression*
#[derive(Debug, Clone)]
pub struct CaseAlternative {
    pub condition: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
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

impl RelationshipDirection {
    pub fn left_string(&self) -> String {
        match self {
            RelationshipDirection::Left => "<-[".to_string(),
            RelationshipDirection::Right => "-[".to_string(),
            RelationshipDirection::Both => "<-[".to_string(),
            RelationshipDirection::None => "-[".to_string(),
        }
    }

    pub fn right_string(&self) -> String {
        match self {
            RelationshipDirection::Left => "]-".to_string(),
            RelationshipDirection::Right => "]->".to_string(),
            RelationshipDirection::Both => "]->".to_string(),
            RelationshipDirection::None => "]-".to_string(),
        }
    }
}

/// Literals
///
/// # Synopsis
/// > - *DoubleLiteral*
/// > - *IntegerLiteral*
/// > - *StringLiteral*
/// > - *BooleanLiteral* := **TRUE** | **FALSE**
/// > - *NullLiteral* := **NULL**
#[derive(Debug, Clone)]
pub enum Literal {
    Double(f64),
    Integer(u64),
    String(String),
    Boolean(bool),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    Null,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Double(value) => f.write_fmt(format_args!("{}", value)),
            Literal::Integer(value) => f.write_fmt(format_args!("{}", value)),
            Literal::String(value) => f.write_fmt(format_args!("{}", value)),
            Literal::Boolean(value) => f.write_str(if *value { "TRUE" } else { "FALSE" }),
            Literal::List(list) => {
                let items = list
                    .iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<String>>();
                f.write_fmt(format_args!("[{}]", &items.join(", ")))
            }
            Literal::Map(entries) => {
                let items = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>();
                f.write_fmt(format_args!("{{{}}}", items.join(", ")))
            }
            Literal::Null => f.write_str("NULL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
}

impl From<ExprKind> for Expr {
    fn from(kind: ExprKind) -> Self {
        Expr { kind }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ExprKind::BinOp(kind, lhs, rhs) => match kind {
                BinOpKind::Or => f.write_fmt(format_args!("{} OR {}", lhs, rhs)),
                BinOpKind::Xor => f.write_fmt(format_args!("{} XOR {}", lhs, rhs)),
                BinOpKind::And => f.write_fmt(format_args!("{} AND {}", lhs, rhs)),
                BinOpKind::Add => f.write_fmt(format_args!("{} + {}", lhs, rhs)),
                BinOpKind::Sub => f.write_fmt(format_args!("{} - {}", lhs, rhs)),
                BinOpKind::Mul => f.write_fmt(format_args!("{} * {}", lhs, rhs)),
                BinOpKind::Div => f.write_fmt(format_args!("{} / {}", lhs, rhs)),
                BinOpKind::Mod => f.write_fmt(format_args!("{} % {}", lhs, rhs)),
                BinOpKind::Pow => f.write_fmt(format_args!("{} ^ {}", lhs, rhs)),
                BinOpKind::Index => f.write_fmt(format_args!("{}[{}]", lhs, rhs)),
                BinOpKind::In => f.write_fmt(format_args!("{} IN {}", lhs, rhs)),
                BinOpKind::Contains => f.write_fmt(format_args!("{} CONTAINS {}", lhs, rhs)),
                BinOpKind::StartsWith => f.write_fmt(format_args!("{} STARTS WITH {}", lhs, rhs)),
                BinOpKind::EndsWith => f.write_fmt(format_args!("{} ENDS WITH {}", lhs, rhs)),
                BinOpKind::Pipe => f.write_fmt(format_args!("{} | {}", lhs, rhs)),
            },
            ExprKind::UnOp(kind, expr) => match kind {
                UnOpKind::Pos => f.write_fmt(format_args!("+{}", expr)),
                UnOpKind::Neg => f.write_fmt(format_args!("-{}", expr)),
                UnOpKind::Not => f.write_fmt(format_args!("NOT {}", expr)),
                UnOpKind::Null => f.write_fmt(format_args!("{} IS NULL", expr)),
                UnOpKind::NotNull => f.write_fmt(format_args!("{} IS NOT NULL", expr)),
                UnOpKind::Parentheses => f.write_fmt(format_args!("({})", expr)),
            },
            ExprKind::Cmp(cmp_expr, tails) => {
                let tail_str: String = tails
                    .iter()
                    .map(|(kind, expr)| format!("{} {}", kind, expr))
                    .collect::<Vec<String>>()
                    .join(" ");
                f.write_fmt(format_args!("{} {}", cmp_expr, &tail_str))
            }
            ExprKind::Lit(lit) => f.write_fmt(format_args!("{}", lit)),
            ExprKind::Variable(name) | ExprKind::PredicateVariable(name) => {
                f.write_str(&name.get_name())
            }
            ExprKind::Case(case_expression, case_alternatives, else_expression) => {
                let head_str = if let Some(expr) = case_expression {
                    format!("CASE {}", expr)
                } else {
                    "CASE".to_string()
                };
                let middle_str = case_alternatives
                    .iter()
                    .map(|case_alternative| {
                        format!(
                            "\n WHEN {} THEN {}",
                            case_alternative.condition, case_alternative.value
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("");
                let tail_str = if let Some(expr) = else_expression {
                    format!("\nELSE {}", expr)
                } else {
                    String::new()
                };
                f.write_fmt(format_args!(
                    "{} {} {} \nEND",
                    &head_str, &middle_str, &tail_str
                ))
            }
            ExprKind::Property(expr, prop) => f.write_fmt(format_args!("{}.{}", expr, prop)),
            ExprKind::Invocation(expr, _, params) => {
                let params_str: String = params
                    .iter()
                    .map(|param| format!("{}", param))
                    .collect::<Vec<String>>()
                    .join(", ");
                f.write_fmt(format_args!("{}({})", expr, &params_str))
            }
            ExprKind::PredicateFunction(kind, expr) => {
                f.write_fmt(format_args!("{}({})", kind, expr))
            }
            ExprKind::SubQuery(kind, expr, where_clause) => {
                let mut transformer = TransformVisitor::new();
                let result = transformer.exec(expr.clone());

                match kind {
                    SubQueryKind::Exists => {
                        // `Exists {Query}`
                        f.write_fmt(format_args!("EXISTS {{{}}}", result))
                    }
                    SubQueryKind::RelationShipsPattern => {
                        // RelationShipsPattern
                        f.write_fmt(format_args!("{}", result))
                    }
                    SubQueryKind::PredicatePattern => {
                        if let Some(where_clause) = where_clause {
                            f.write_fmt(format_args!("{} WHERE {}", result, where_clause))
                        } else {
                            f.write_fmt(format_args!("{}", result))
                        }
                    }
                }
            }
            ExprKind::Label(expr, label) => f.write_fmt(format_args!("{}:{}", expr, label)),
            ExprKind::FilterExpression(var, in_expr, where_expr) => {
                if let Some(where_expr) = where_expr {
                    f.write_fmt(format_args!("{} IN {} WHERE {}", var, in_expr, where_expr))
                } else {
                    f.write_fmt(format_args!("{} IN {}", var, in_expr))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binop_display() {
        let ops = vec![
            BinOpKind::Or,
            BinOpKind::Xor,
            BinOpKind::And,
            BinOpKind::Add,
            BinOpKind::Sub,
            BinOpKind::Mul,
            BinOpKind::Div,
            BinOpKind::Mod,
            BinOpKind::Pow,
            BinOpKind::Index,
            BinOpKind::In,
            BinOpKind::Contains,
            BinOpKind::StartsWith,
            BinOpKind::EndsWith,
        ];
        let results = vec![
            "a OR b",
            "a XOR b",
            "a AND b",
            "a + b",
            "a - b",
            "a * b",
            "a / b",
            "a % b",
            "a ^ b",
            "a[b]",
            "a IN b",
            "a CONTAINS b",
            "a STARTS WITH b",
            "a ENDS WITH b",
        ];

        for (op, res) in ops.iter().zip(results.iter()) {
            let l_val = Expr {
                kind: ExprKind::Variable(Variable::new("a".to_string())),
            };
            let r_val = Expr {
                kind: ExprKind::Variable(Variable::new("b".to_string())),
            };
            let expr = Expr {
                kind: ExprKind::BinOp(*op, Box::new(l_val), Box::new(r_val)),
            };
            assert_eq!(format!("{}", expr), res.to_string());
        }
    }

    #[test]
    fn test_unop_display() {
        let ops = vec![
            UnOpKind::Pos,
            UnOpKind::Neg,
            UnOpKind::Not,
            UnOpKind::Null,
            UnOpKind::NotNull,
            UnOpKind::Parentheses,
        ];
        let results = vec!["+a", "-a", "NOT a", "a IS NULL", "a IS NOT NULL", "(a)"];

        for (op, res) in ops.iter().zip(results.iter()) {
            let val = Expr {
                kind: ExprKind::Variable(Variable::new("a".to_string())),
            };
            let expr = Expr {
                kind: ExprKind::UnOp(*op, Box::new(val)),
            };
            assert_eq!(format!("{}", expr), res.to_string());
        }
    }

    // #[test]
    // fn test_property_display() {
    // let l_val = Expr {
    //     kind: ExprKind::Variable("a".to_string()),
    //     span: Span(0, 0),
    // };
    // let expr = Expr {
    //     kind: ExprKind::Property(Box::new(l_val), "prop".to_string()),
    // };
    // assert_eq!(format!("{}", expr), "a.prop".to_string());
    // }
}
