use std::path::PathBuf;

use crate::{
    ast::{CypherGenerator, CypherNode, TransformVisitor},
    common::{Log, RandomGenerator, OutputWriter},
    config::CypherConfig,
    db::{AtlasConfig, AtlasConnection},
    meta::GraphSchema,
};
use serde_json::Value;
use rpc::atlas::ExecRequest;
use tonic::Request;

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
    async fn load_atlas(&mut self, atlas: AtlasConfig) {
        self.atlas_connection = Some(AtlasConnection::new(atlas).await);
        println!("\nConnect Success!\n");
    }

}

impl Driver {
    /// ast tree construct
    pub fn construct(&mut self) -> CypherNode {
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

impl Driver {
    /// Load the Config.
    /// Return the result as a string.
    pub async fn load(&mut self, atlas_path: PathBuf) -> Result<(), String> {
        // atlas information
        let json = std::fs::read_to_string(atlas_path).unwrap();
        let atlas = serde_json::from_str::<AtlasConfig>(&json).unwrap();
        println!("Atlas Config Connection: \n{:?}", atlas);
        self.load_atlas(atlas).await;

        Ok(())
    }

    /// databse execution
    pub async fn execute(&mut self) -> Result<(), String> {
        // log_record recording intermediate information
        let mut log_record = Log::new();

        

        let mut results = Vec::new();

        // while current queries < max_queries.
        while self.queries < self.cypher_config.max_queries {
            // generator the ast tree and string.
            let cypher_ast = self.construct();

            // transform ast tree to string.
            let cypher_string = self.transfrom(Box::new(cypher_ast.clone()));
            

            // print queries instead of executing them
            if  self.cypher_config.dry_run {
                println!("CypherString:\n{}", cypher_string);
            }

            // dump generated ASTs for debugging.
            if self.cypher_config.dump_all_graphs {
                println!("CypherAST:\n{:?}", cypher_ast);
            }
            
            log_record.execute(Box::new(cypher_ast));

            // query number add 1
            self.queries += 1;

            // if connect to AtlasGraph
            if self.atlas_connection.is_some() {
                let mut atlas_client = self.atlas_connection.as_ref().unwrap().client.clone();
                let session_id = self.atlas_connection.as_ref().unwrap().session_id.clone();

                // if self.cypher_config

                //let session_id = self
                let res = atlas_client
                    .exec(Request::new(ExecRequest {
                        session_id: session_id.clone(),
                        statement: cypher_string.clone(),
                    }))
                    .await
                    .unwrap()
                    .into_inner()
                    .result;

                // println!("\n{}", res);

                let mut v: Value = serde_json::from_str(res.as_str()).unwrap();
                let errors = v["errors"].take();
                let record = serde_json::from_value::<Vec<String>>(errors).unwrap();
                if !record.is_empty() {
                    results.push((cypher_string, record));
                }
            }
        }

        // verbose
        if let Some(path) = &self.cypher_config.verbose {
            let mut output = OutputWriter::new(path.to_string());
            for (cypher, errors) in results {
                output.write_errors(cypher, errors);
            }
        }
        
        // print report.
        log_record.report();

        Ok(())
    }
}
