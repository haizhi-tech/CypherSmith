use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[clap(
    name = "OpenCypher Generator",
    author = "AtlasGraph Authors",
    after_help = r#"# Examples

## import schema
$ cypher-smith --schema schema.json

"#
)]
pub struct ArgsConfig {
    // #[clap(
    //     short,
    //     long,
    //     default_value = "127.0.0.1:21021",
    //     value_name = "HOST:PORT",
    //     help = "atlas server addr"
    // )]
    // #[doc(alias = "server_addr")]
    // pub address: std::net::SocketAddrV4,
    // #[clap(short, long, default_value = "root", value_name = "STRING")]
    // pub username: String,
    // #[clap(short, long, default_value = "root", value_name = "STRING")]
    // pub password: String,
    /// import schema
    #[clap(short, long, value_name = "PATH", help = "path of schema.json")]
    pub schema: Option<PathBuf>,
}

impl Default for ArgsConfig {
    fn default() -> Self {
        Self::parse_from::<&[&'static str], &&'static str>(&[])
    }
}
