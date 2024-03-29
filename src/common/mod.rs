mod diagnostic;
mod expr;
mod log;
mod rand;
mod typedef;
mod util;
mod variable;

pub use self::rand::RandomGenerator;
// pub use expr::{
//     Expr, ExprKind, Expression, Literal, NameSpace, NodeLabel, PropertyExpression,
//     RelationshipDirection, SchemaName, Variable, VariableGenerator,
// };
pub use diagnostic::*;
pub use expr::*;
pub use log::Log;
pub use typedef::*;
pub use util::*;
pub use variable::{DataKind, Variable, VariableGenerator};

pub mod constants {
    pub const DEFAULT_LOOP_LIMIT: i32 = 3;
    pub const DEFAULT_EXPRESSION_LIMIT: i32 = 5;
    pub const DEFAULT_QUERY_LIMIT: i32 = 15;
    // pub const DEFAULT_RETRY_LIMIT: i32 = 20;
}

#[cfg(test)]
mod tests {
    use super::{Log, VariableGenerator};
    use crate::ast::{CypherNode, LogVisitor};

    #[test]
    fn test_get_info() {
        let mut test_logger = Log::new();
        let pattern_parts = vec![Box::new(CypherNode::PatternPart {
            var: None,
            pattern_element: Box::new(CypherNode::PatternElement {
                parenthesis: false,
                pattern_element: (
                    Box::new(CypherNode::NodePattern {
                        var: None,
                        vertex_labels: vec![],
                        properties: None,
                    }),
                    vec![],
                ),
            }),
        })];

        let match_clause = CypherNode::UpdatingClause {
            updating_clause: Box::new(CypherNode::Match {
                is_optional: false,
                pattern: Box::new(CypherNode::Pattern { pattern_parts }),
                where_clause: None,
            }),
        };

        let test_cypher_node = CypherNode::Query {
            query: Box::new(CypherNode::RegularQuery {
                single_query: Box::new(CypherNode::SingleQuery {
                    part_query: Box::new(CypherNode::SinglePartQuery {
                        reading_clauses: Vec::new(),
                        updating_clauses: vec![Box::new(match_clause)],
                        return_clause: None,
                    }),
                }),
                union_all: vec![],
            }),
        };

        let (sum_nodes, max_level) = test_logger.visit(Box::new(test_cypher_node));
        println!(
            "Stat Result:\ntotal nodes: {}\nmax level: {}",
            sum_nodes, max_level
        );
    }

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

    // #[test]
    // fn test_schema_name() {
    //     let mut random_gen = RandomGenerator::new();
    //     let new_schema_name = SchemaName::new(&mut random_gen);
    //     println!("{:?}", new_schema_name);
    // }
}
