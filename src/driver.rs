use crate::ast::{CypherGenerator, CypherNode, TransformVisitor};
use crate::meta::GraphSchema;

#[derive(Default)]
pub struct Driver {
    queries: u32,
    graph_schema: GraphSchema,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            queries: 0,
            graph_schema: GraphSchema::default(),
        }
    }

    pub fn load_schema(&mut self, schema: GraphSchema) -> GraphSchema {
        self.graph_schema = schema;
        self.graph_schema.clone()
    }

    // ast tree construct
    pub fn execute(&self) -> (CypherNode, String) {
        // let transform = TransformVisitor::new();
        // let mut ast_generator = CypherGenerator::new();
        let mut ast_generator = CypherGenerator::new_schema(&self.graph_schema);
        ast_generator.visit();
        ast_generator.test_match_clause()
    }

    // ast tree transfrom to cypher string.
    pub fn transfrom(&self, cypher_node: Box<CypherNode>) -> String {
        let mut transformer = TransformVisitor::new();
        transformer.exec(cypher_node)
    }

    pub fn add_query(&mut self) {
        self.queries += 1u32;
    }

    // print transfrom.
    pub fn print(&self) {}
}
