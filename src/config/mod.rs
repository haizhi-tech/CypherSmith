use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// GraphSchema Config.
#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[clap(
    name = "OpenCypher Generator",
    author = "AtlasGraph Authors",
    after_help = r#"# Examples

## import schema and basic config.
$ cypher-smith --schema schema.json --config config.json

## import schema and atlas config.
$ cypher-smith --schema schema.json --config config.json --atlas atlas.json

"#
)]
pub struct ArgsConfig {
    #[clap(short, long, value_name = "PATH", help = "schema information")]
    pub schema: Option<PathBuf>,
    #[clap(short, long, value_name = "PATH", help = "basic config information")]
    pub config: Option<PathBuf>,
    #[clap(short, long, value_name = "PATH", help = "basic config information")]
    pub atlas: Option<PathBuf>,
}

impl Default for ArgsConfig {
    fn default() -> Self {
        Self::parse_from::<&[&'static str], &&'static str>(&[])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CypherConfig {
    // StandaloneCall
    pub call_query: bool,
    pub max_queries: u32,
    pub dry_run: bool,
    pub verbose: Option<String>,
    pub dump_all_graphs: bool,
}

impl Default for CypherConfig {
    fn default() -> Self {
        CypherConfig {
            call_query: false,
            max_queries: 100,
            dry_run: true,
            verbose: None,
            dump_all_graphs: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CypherConfig;

    #[test]
    fn test_deserialize() {
        let cypher_config = CypherConfig {
            call_query: false,
            max_queries: 100,
            dry_run: true,
            verbose: Some("test".to_string()),
            dump_all_graphs: false,
        };

        println!("{:?}", cypher_config);
    }
}
