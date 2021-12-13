use super::expr::{
    Expression, IntegerLiteral, NodeLabel, Paramter, RelationshipDirection, ReserverdWord,
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

    /// SingleQuery
    SingleQuery {
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    },

    /// ReadingClause
    ///
    /// Match or Unwind or InqueryCall
    ReadingClause {
        match_clause: Option<Box<CypherNode>>,
    },

    /// Return clause
    ///
    /// 'return' ProjectionBody
    ///  ProjectionBody -> ProjectionItems
    Return {
        projection_body: Vec<CypherNode>,
    },

    /// ProjectionItem
    ///
    ProjectionItem {
        expressions: Vec<(Expression, Option<Variable>)>,
    },

    /// Match
    ///
    Match {
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Box<CypherNode>>,
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
        var: Variable,
        pattern_element: Box<CypherNode>,
    },

    /// PatternElement
    ///
    /// Vec<(NodePattern, Vec<PatternElementChain>)>
    PatternElement {
        pattern_element: Vec<(Box<CypherNode>, Vec<Box<CypherNode>>)>,
    },

    /// NodePattern
    NodePattern {
        var: Option<Variable>,
        labels: Vec<NodeLabel>,
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
