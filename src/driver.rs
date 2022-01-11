use crate::{
    ast::{CypherGenerator, CypherNode, TransformVisitor},
    common::RandomGenerator,
    config::CypherConfig,
    meta::GraphSchema,
    db::{AtlasConfig, AtlasConnection},
};

#[derive(Default)]
pub struct Driver {
    queries: u32,
    random: RandomGenerator,
    graph_schema: GraphSchema,
    cypher_config: CypherConfig,
    atlas_connection: Option<AtlasConnection>,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            queries: 0,
            random: RandomGenerator::default(),
            graph_schema: GraphSchema::default(),
            cypher_config: CypherConfig::default(),
            atlas_connection: None,
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

    /// Connect to AtlasGraph.
    pub async fn load_atlas(&mut self, atlas: AtlasConfig) {
        self.atlas_connection = Some(AtlasConnection::new(atlas).await);
    }
}

impl Driver {
    /// ast tree construct
    pub fn execute(&mut self) -> CypherNode {
        let mut ast_generator = CypherGenerator::new_schema(&self.graph_schema);
        if self.cypher_config.call_query && self.random.d9() > 7 {
            ast_generator.call_query()
        } else {
            ast_generator.visit()
        }
    }

    /// ast tree transfrom to cypher string.
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
