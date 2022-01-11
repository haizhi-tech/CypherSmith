use crate::{
    ast::{CypherGenerator, CypherNode, TransformVisitor},
    common::RandomGenerator,
    config::{ArgsConfig, CypherConfig},
    db::{AtlasConfig, AtlasConnection},
    meta::GraphSchema,
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

    fn load_schema(&mut self, schema: GraphSchema) -> GraphSchema {
        self.graph_schema = schema;
        self.graph_schema.clone()
    }

    fn load_config(&mut self, config: CypherConfig) -> CypherConfig {
        self.cypher_config = config;
        self.cypher_config.clone()
    }

    /// Connect to AtlasGraph.
    async fn load_atlas(&mut self, atlas: AtlasConfig) {
        self.atlas_connection = Some(AtlasConnection::new(atlas).await);
        println!("\nConnect Success!\n");
    }

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

impl Driver {}

impl Driver {
    /// Load the Config.
    /// Return the result as a string.
    pub async fn load(&mut self, args_config: ArgsConfig) -> Result<(), String> {
        if args_config.schema.is_none() {
            // eprintln!("[WARNING] Please provide schema information!\n\tuse `cypher_smith --help` to find out example usage");
            return Err("[WARNING] Please provide schema information!\n\tuse `cypher_smith --help` to find out example usage".to_string());
        }

        if let Some(ref schema_path) = args_config.schema {
            let schema_path = schema_path.clone();
            let json = std::fs::read_to_string(schema_path).unwrap();
            let schema = serde_json::from_str::<GraphSchema>(&json).unwrap();
            println!("Input schema information: \n{:?}", schema);
            self.load_schema(schema);
        }

        if let Some(ref config_path) = args_config.config {
            let config_path = config_path.clone();
            let json = std::fs::read_to_string(config_path).unwrap();
            let config = serde_json::from_str::<CypherConfig>(&json).unwrap();
            println!("\nInput basic config information: \n{:?}", config);
            self.load_config(config);
        }

        if let Some(ref atlas_path) = args_config.atlas {
            let atlas_path = atlas_path.clone();
            let json = std::fs::read_to_string(atlas_path).unwrap();
            let atlas = serde_json::from_str::<AtlasConfig>(&json).unwrap();
            println!("Atlas Config Connection: \n{:?}", atlas);
            self.load_atlas(atlas).await;
        }

        Ok(())
    }
}
