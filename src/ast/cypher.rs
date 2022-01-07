use crate::common::{
    Expr, FieldValue, NameSpace, Property, PropertyExpression, RelationshipDirection, Variable,
};
use crate::meta::Label;

use paste::paste;

macro_rules! cypher_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    )* ) => {
        paste! {
            #[derive(Clone)]
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

            /// The visitor trait for `CypherNode`.
            ///
            /// You should implementation api functions for every variant.
            /// If you are only interested in some subset of variants, just use the `match` statement.
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

            pub trait LogVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self $(, $param: $type)* )  -> Self::Output;
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

    ( FORMAT($formatter: ident) $param:ident : $type:ty ) => {
        $formatter.field(stringify!($param), $param);
    };

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
        procedure: Box<CypherNode>,
        yield_items: (bool, Option<Box<CypherNode>>),
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
        where_clause: Option<Expr>,
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
        skip: Option<Expr>,
        limit: Option<Expr>,
    },

    /// ProjectionItems
    ///
    /// Projectionitem+ | *,(projectionitem)*
    ProjectionItems {
        // is_all = true: *
        is_all: bool,
        // expression as variable.
        expressions: Vec<(Expr, Option<Variable>)>,
    },

    /// Order: order by sort_items(expression (asc|desc|...|)?)+
    Order {
        sort_items: Vec<(Expr, Option<String>)>,
    },

    /// Match
    ///
    Match {
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Expr>,
    },

    /// Unwind : UNWIND Expression AS Variable
    Unwind {
        expression: Expr,
        variable: Variable,
    },

    /// InQueryCall
    InQueryCall {
        explicit_proceduce_invocation: Box<CypherNode>,
        yield_items: Option<Box<CypherNode>>,
    },

    /// Create
    Create {
        pattern: Box<CypherNode>,
    },

    /// Merge
    Merge {
        pattern_part: Box<CypherNode>,
        merge_actions: Vec<(String, Box<CypherNode>)>,
    },

    /// Delete
    Delete {
        is_detach: bool,
        expressions: Vec<Expr>,
    },

    /// Set
    Set {
        property_set: Vec<(PropertyExpression, Expr)>,
        variable_set: Vec<(Variable, Expr)>,
        variable_add: Vec<(Variable, Expr)>,
        label_set: Vec<(Variable, Vec<Label>)>,
    },

    /// ExplicitProcedureInvocation
    ExplicitProcedureInvocation {
        // todo: need to implementation NameSpace.SymbolicName ed: atlas.shortestpath()
        procedure_name: (NameSpace, Variable),
        expressions: Vec<Expr>,
    },

    /// ImplicitProcedureInvocation
    ImplicitProcedureInvocation {
        // todo: need to implementation NameSpace.SymbolicName ed: atlas.shortestpath()
        procedure_name: (NameSpace, Variable),
    },

    /// YieldItems
    YieldItems {
        // todo: need to modify ProecdureResultField result.
        yield_items: Vec<(Option<Variable>, Variable)>,
        where_clause: Option<Expr>,
    },

    /// Remove
    Remove {
        variable_remove: Vec<(Variable, Vec<Label>)>,
        property_remove: Vec<PropertyExpression>,
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
        parenthesis: bool,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    },

    /// NodePattern: properties: Literal|Parameter
    NodePattern {
        var: Option<Variable>,
        vertex_labels: Vec<Label>,
        properties: Option<(Property, FieldValue)>,
    },

    /// RelationshipPattern: [variable :label|:label * 1..2 properties]
    RelationshipPattern {
        direction: RelationshipDirection,
        var: Option<Variable>,
        edge_labels: Vec<Label>,
        is_range: bool,
        range: (Option<i32>, Option<(bool, Option<i32>)>),
        properties: Option<(Property, FieldValue)>,
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
