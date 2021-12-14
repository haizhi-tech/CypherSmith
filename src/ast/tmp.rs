use paste::paste;

macro_rules! expression_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    )* ) => {
        paste! {
            pub enum ExpressionNode {
                $(
                    $(#[doc = $node_doc])*
                    $name {
                        $( $(#[doc = $param_doc])* $param : $type ,)*
                    },
                )*
            }

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

            impl std::fmt::Debug for ExpressionNode {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            ExpressionNode::$name { $( $param, )* } => {
                                let mut struct_formatter = f.debug_struct(stringify!($name));
                                $( expression_nodes_impl!( FORMAT(struct_formatter) $param: $type); )*
                                struct_formatter.finish()
                            },
                        )*
                    }
                }
            }
        }
    };


    ( FORMAT($formatter: ident) $param:ident : $type:ty ) => {
        $formatter.field(stringify!($param), $param);
    };

}

expression_nodes_impl! {

    /// Expression: OrExpression
    Expression {
        or_expression: Box<ExpressionNode>,
    },

    /// OrExpression: Vec<XorExpression>
    OrExpression {
        xor_expressions: Vec<Box<ExpressionNode>>,
    },

    /// XorExpression: Vec<AndExpression>
    XorExpression {
        xor_expressions: Vec<Box<ExpressionNode>>,
    },

    /// AndExpression: Vec<NotExpression>
    AndExpression {
        not_expressions: Vec<Box<ExpressionNode>>,
    },

    /// NotExpression: Not? ComparsionExpression
    NotExpression {
        is_not: bool,
        comparsion_expression: Box<ExpressionNode>,
    },

    /// ComparsionExpression: AddOrSubtractExpression + Vec<PartialComparisonExpression>
    ComparisonExpression {
        add_or_subtract_expression: Box<ExpressionNode>,
        part_comparison_expression: Vec<Box<ExpressionNode>>,
    },

    /// PartialComparisonExpression: = <> < > <= >= AddOrSubtractExpression
    PartialComparisonExpression {
        partial_kind: Option<ExprKind>,
        add_or_subtract_expression: Box<ExpressionNode>,
    },

    /// AddOrSubtractExpression: MultiplyDivideModuloExpression (+/- MultiplyDivideModuloExpression)*
    AddOrSubtractExpression {
        base_expression: Box<ExpressionNode>,
        expressions: Vec<(ExprKind, Box<ExpressionNode>)>,
    },

    /// MultiplyDivideModuloExpression: PowerOfExpression (*///% PowerOfExpression)*
    MultiplyDivideModuloExpression {
        base_expression: Box<ExpressionNode>,
        expressions: Vec<(ExprKind, Box<ExpressionNode>)>,
    },

    /// PowerOfExpression: UnaryAddOrSubtractExpression (^ UnaryAddOrSubtractExpression)*
    PowerOfExpression {
        base_expression: Box<ExpressionNode>,
        expressions: Vec<(ExprKind, Box<ExpressionNode>)>,
    },

    /// UnaryAddOrSubtractExpression: (+/-)* StringListNullOperatorExpression
    ///
    UnaryAddOrSubtractExpression {
        operators: Vec<ExprKind>,
        expressions: Box<ExpressionNode>,
    },

    /// StringListNullOperatorExpression: PropertyOrLabelsExpression, (StringOperatorExpression|ListOperatorExpression|NullOperatorExpression)*
    StringListNullOperatorExpression {
        property_expression: Box<ExpressionNode>,
        expressions: Vec<Box<ExpressionNode>>,
    },

    /// PropertyOrLabelsExpression: Atom, (PropertyLookup)*, (NodeLabels)+
    PropertyOrLabelsExpression {
        // atom: ,
        property_lookup: Vec<String>,
        node_labels: Option<Vec<super::expr::NodeLabel>>,
    },

    /// StringOperatorExpression: (STARTS WITH | ENDS WITH | CONTAINS)? PropertyOrLabelsExpression
    StringOperatorExpression {
        operators: Option<ExprKind>,
        expr: Box<ExpressionNode>,
    },

    /// ListOperatorExpression:
    ListOperatorExpression {
        in_expr: Option<Box<ExpressionNode>>,
        exprs: Option<Vec<Box<ExpressionNode>>>,
    },

    /// NullOperatorExpression: IS NULL/ IS NOT NULL.
    NullOperatorExpression {
        is_null: bool,
    },
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
    List(Vec<Box<ExpressionNode>>),
    Map(Vec<(String, Box<ExpressionNode>)>),
    Null,
}

// pub ComparisonExpression
