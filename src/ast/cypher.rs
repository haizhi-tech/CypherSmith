use super::expr::{
    Expression, IntegerLiteral, Paramter, RelationshipDirection, ReserverdWord, SymbolicName,
    Variable,
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
                    fn [<visit_ $name:snake>](&mut self $(, $param: &$type)* ) -> Self::Output;
                )*

                fn visit(&mut self, node: &CypherNode) -> Self::Output {
                    match node {
                        $(
                            CypherNode::$name { $( $param ,)* } => self.[<visit_ $name:snake>]($($param),*),
                        )*
                    }
                }
            }
        }
    };

    // ( $(
    //     $(#[doc = $variant_doc:expr])*
    //     $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    // )* )
}

cypher_nodes_impl! {

    /// Base Query: RegularQuery || StandaloneCall
    Query {
        query: Box<CypherNode>,
    },

    /// RegularQuery
    RegularQuery {
        single_query: Box<CypherNode>,
        union_all: Vec<Box<CypherNode>>,
    },
}

pub enum Query {
    RegularQuery(RegularQuery),
    StandaloneCall,
}
pub struct RegularQuery {
    single_query: SingleQuery,
    union_all: Vec<UnionQuery>,
}

pub enum SingleQuery {
    SinglePartQuery(SinglePartQuery),
    MultiPartQuery(MultiPartQuery),
}

pub struct UnionQuery {}

/// to part: one is
/// readingclause* return
/// updatingclause+ [return]
pub struct SinglePartQuery {
    reading_clauses: Vec<ReadingClause>,
    updating_clauses: Vec<UpdatingClause>,
    return_clause: Option<Return>,
}
pub struct MultiPartQuery {}

pub enum ReadingClause {
    Match(Match),
    Unwind,
    InQueryCall,
}

/// match: [optional] match pattern [where]
pub struct Match {
    is_optional: bool,
    pattern: Pattern,
    where_clause: Option<WhereClause>,
}

pub struct Pattern {
    pattern_parts: Vec<PatternPart>,
}

pub enum PatternPart {
    AnonymousPattternPart(AnonymousPattternPart),
    VariableAnonymousPattternPart(VariableAnonymousPattternPart),
}

pub struct VariableAnonymousPattternPart {
    variable: Variable,
    anonymous_patttern_part: AnonymousPattternPart,
}

pub struct AnonymousPattternPart {
    pattern_element: PatternElement,
}

/// PatternElement: ( PatternElement ) | nodepattern (patternelementchain)*
pub struct PatternElement {
    is_self: Box<PatternElement>,
    node_pattern: NodePattern,
    pattern_element_chain: PatternElementChain,
}

pub struct PatternElementChain {
    relationship_pattern: RelationshipPattern,
    node_pattern: NodePattern,
}

/// relationshippattern: <-[ ]->
/// <- [] -
/// - [] ->
/// - [] -
pub struct RelationshipPattern {
    relaionship_direction: RelationshipDirection,
    relationship_detail: RelationShipDetail,
}

pub struct RelationShipDetail {
    variable: Option<Variable>,
    relationship_types: Option<Vec<RealTypeName>>,
    range_literal: Option<RangeLiteral>,
    properties: Option<Properties>,
}

pub struct Properties {
    map_literal: MapLiteral,
    paramter: Paramter,
}

pub struct MapLiteral {
    properties_expression: Vec<(PropertyKeyName, Expression)>,
}

pub enum PropertyKeyName {
    SchemaName(SchemaName),
}

pub struct RangeLiteral {
    range_literal: Vec<IntegerLiteral>,
}

// realtypename: schema name
pub struct RealTypeName {
    schema_name: SchemaName,
}

pub enum SchemaName {
    SymbolicName(SymbolicName),
    ReservedWord(ReserverdWord),
}

pub struct NodePattern {}

pub struct WhereClause {}

pub struct UpdatingClause {}

pub struct Return {}

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
