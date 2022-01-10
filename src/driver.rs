use crate::ast::{CypherGenerator, CypherNode, TransformVisitor};
use crate::common::RandomGenerator;
use crate::config::CypherConfig;
use crate::meta::GraphSchema;

#[derive(Default)]
pub struct Driver {
    queries: u32,
    random: RandomGenerator,
    graph_schema: GraphSchema,
    cypher_config: CypherConfig,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            queries: 0,
            random: RandomGenerator::default(),
            graph_schema: GraphSchema::default(),
            cypher_config: CypherConfig::default(),
        }
    }

    pub fn load_schema(&mut self, schema: GraphSchema) -> GraphSchema {
        self.graph_schema = schema;
        self.graph_schema.clone()
    }

    pub fn load_config(&mut self, config: CypherConfig) -> CypherConfig {
        self.cypher_config = config;
        self.cypher_config.clone()
    }

    // ast tree construct
    pub fn execute(&mut self) -> CypherNode {
        let mut ast_generator = CypherGenerator::new_schema(&self.graph_schema);
        if self.cypher_config.call_query && self.random.d9() > 7 {
            ast_generator.call_query()
        } else {
            ast_generator.visit()
        }
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
