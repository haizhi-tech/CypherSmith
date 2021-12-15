use super::expr::{
    Expression, IntegerLiteral, NodeLabel, Paramter, Properties, RelationshipDirection,
    SymbolicName, Variable,
};

use paste::paste;

macro_rules! cypher_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    )* ) => {
        paste! {
            pub enum CypherNode {
                $(
                    $(#[doc = $node_doc])*
                    $name {
                        $( $(#[doc = $param_doc])* $param : $type ,)*
                    },
                )*
            }

            pub trait CypherNodeVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self) -> Self::Output;
                )*

                fn visit(&mut self) -> Self::Output {
                    self.visit_query()
                }
            }

            // pub trait ConvertVisitor {
            //     type Output;

            //     $(
            //         fn [<visit_ $name:snake>](&self $(, $param: $type)* ) -> Self::Output;
            //     )*


            //     // fn visit(&mut self, node: &CypherNode) -> Self::Output {
            //     //     match node {
            //     //         $(
            //     //             CypherNode::$name { $( $param ,)* } => self.[<visit_ $name:snake>]($($param),*),
            //     //         )*
            //     //     }
            //     // }

            //     fn visit(&mut self, node: &CypherNode) -> Self::Output {
            //         match node {
            //             $(
            //                 CypherNode::$name { $( $param ,)* } => self.[<visit_ $name:snake>]($($param),*),
            //             )*
            //         }
            //     }
            // }

            pub trait ConvertVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self $(, $param: $type)* ) -> Self::Output;
                )*

                fn visit(&mut self, node: impl Into<CypherNode>) -> Self::Output {
                    let node: CypherNode = node.into();
                    match node {
                        $(
                            CypherNode::$name { $( $param ,)* } => self.[<visit_ $name:snake>]($($param),*),
                        )*
                    }
                }
            }

            impl std::fmt::Debug for CypherNode {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                    $(
                        CypherNode::$name { $( $param, )* } => {
                            let mut struct_formatter = f.debug_struct(stringify!($name));
                            $( cypher_nodes_impl!( FORMAT(struct_formatter) $param: $type); )*
                            struct_formatter.finish()
                        },
                    )*
                    }
                }
            }
        }
    };

    // ( FORMAT($formatter: ident) $param:ident : Box<CypherNode> ) => {
    //     // do not recursively format subplan
    // };
    ( FORMAT($formatter: ident) $param:ident : $type:ty ) => {
        $formatter.field(stringify!($param), $param);
    };

    // ( $(
    //     $(#[doc = $variant_doc:expr])*
    //     $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    // )* )
}

cypher_nodes_impl! {

    /// Base Query: RegularQuery | StandaloneCall
    Query {
        query: Box<CypherNode>,
    },

    /// RegularQuery
    RegularQuery {
        single_query: Box<CypherNode>,
        union_all: Vec<Box<CypherNode>>,
    },

    /// StandaloneCall
    StandaloneCall {

    },

    /// SingleQuery
    SingleQuery {
        part_query: Box<CypherNode>,
    },

    /// SinglePartQuery
    SinglePartQuery {
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    },

    /// MultiPartQuery
    MultiPartQuery {
        multi_part: Vec<(Vec<Box<CypherNode>>, Vec<Box<CypherNode>>, Box<CypherNode>)>,
        single_part: Box<CypherNode>,
    },

    /// WithQuery
    With {
        projection_body: Box<CypherNode>,
        where_clause: Option<Expression>,
    },

    /// Union
    Union {
        union_all: Option<(bool, Box<CypherNode>)>,
    },

    /// ReadingClause
    ///
    /// Match or Unwind or InqueryCall
    ReadingClause {
        reading_clause: Box<CypherNode>,
    },

    /// UpdatingClause
    UpdatingClause {
        updating_clause: Box<CypherNode>,
    },

    /// Return clause
    ///
    /// 'return' ProjectionBody
    ///  ProjectionBody -> ProjectionItems
    Return {
        projection_body: Box<CypherNode>,
    },

    /// ProjectionBody: DISTINCT? ProjectionItems Order? Skip? Limit?
    ProjectionBody {
        is_distinct: bool,
        projection_items: Box<CypherNode>,
        order: Option<Box<CypherNode>>,
        skip: Option<Expression>,
        limit: Option<Expression>,
    },

    /// ProjectionItems
    ///
    /// Projectionitem+ | *,(projectionitem)*
    ProjectionItems {
        // is_all = true: *
        is_all: bool,
        // expression as variable.
        expressions: Vec<(Expression, Option<Variable>)>,
    },

    /// Order: order by sort_items(expression (asc|desc|...|)?)+
    Order {
        sort_items: Vec<(Expression, Option<String>)>,
    },

    /// Match
    ///
    Match {
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Expression>,
    },

    /// Unwind : UNWIND Expression AS Variable
    Unwind {
        expression: Expression,
        variable: Variable,
    },

    /// InQueryCall
    InQueryCall {
        explicit_proceduce_invocation: Box<CypherNode>,
        yield_items: Vec<Box<CypherNode>>,
    },

    /// Create
    Create {
        pattern: Box<CypherNode>,
    },

    /// Merge
    Merge {

    },

    /// Delete
    Delete {

    },

    /// Set
    Set {

    },

    /// Remove
    Remove {

    },

    /// Pattern
    ///
    /// Vec<PatternPart>
    Pattern {
        pattern_parts: Vec<Box<CypherNode>>,
    },

    /// PatternPart
    ///
    /// Variable = AnonymousPatternPart
    /// AnonymousPatternPart : PatternElement
    PatternPart {
        var: Option<Variable>,
        pattern_element: Box<CypherNode>,
    },

    /// PatternElement
    ///
    /// Vec<(NodePattern, Vec<(RelationShipPattern, NodePattern)>)>
    PatternElement {
        parentheses: i32,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    },

    /// NodePattern: properties: Literal|Parameter
    NodePattern {
        var: Option<Variable>,
        vertex_labels: Vec<NodeLabel>,
        properties: Option<Properties>,
    },

    /// RelationshipPattern: [variable :label|:label * 1..2 properties]
    RelationshipPattern {
        direction: RelationshipDirection,
        var: Option<Variable>,
        edge_labels: Vec<NodeLabel>,
        range: (Option<i32>, Option<i32>),
        properties: Option<Properties>,
    },
}

impl From<Box<CypherNode>> for CypherNode {
    fn from(x: Box<CypherNode>) -> Self {
        // NOTE: deref-move syntax only works for Box<T>
        *x
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::common::RandomGenerator;

    #[test]
    pub fn construct() {
        // let single_query  = SingleQuery{};
        // let regular_query = RegularQuery{ single_query: Box::new(single_query) , union_all:vec![]};
        // let query = Query::RegularQuery(regular_query);
        // println!("{}", query);
    }
}
