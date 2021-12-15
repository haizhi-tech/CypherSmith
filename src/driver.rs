use crate::ast::{CypherGenerator, CypherNode, TransformVisitor};
use crate::common::RandomGenerator;

#[derive(Default)]
pub struct Driver {
    random_generator: RandomGenerator,
    queries: u32,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            random_generator: RandomGenerator::new(),
            queries: 0,
        }
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
