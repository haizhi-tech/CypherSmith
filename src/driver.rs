use crate::ast::{CypherGenerator, CypherNode, TransformVisitor};
use crate::meta::GraphSchema;

#[derive(Default)]
pub struct Driver {
    queries: u32,
    //graph_schema: GraphSchema,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            queries: 0,
        }
    }

    pub fn new_schema() {

    }

    // ast tree construct
    pub fn execute(&self) -> CypherNode {
        // let transform = TransformVisitor::new();
        let mut ast_generator = CypherGenerator::new();
        ast_generator.visit()
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
