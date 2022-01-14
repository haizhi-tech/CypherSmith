use cypher_smith::{ArgsConfig, CypherConfig, Driver, GraphSchema};

fn main() {
    // get user config.
    let config = <ArgsConfig as clap::Parser>::parse();

    if config.schema.is_none() || config.config.is_none() {
        eprintln!("[WARNING] Please provide schema and basic config information!\n\tuse `cypher_smith --help` to find out example usage");
        return;
    }

    // get the label name and so on.
    let mut driver = Driver::new();

    // schema information
    if let Some(ref schema_path) = config.schema {
        let schema_path = schema_path.clone();
        let json = std::fs::read_to_string(schema_path).unwrap();
        let schema = serde_json::from_str::<GraphSchema>(&json).unwrap();
        println!("Input schema information: \n{:?}", schema);
        driver.load_schema(schema);
    }

    // basic config information
    if let Some(ref config_path) = config.config {
        let config_path = config_path.clone();
        let json = std::fs::read_to_string(config_path).unwrap();
        let config = serde_json::from_str::<CypherConfig>(&json).unwrap();
        println!("\nInput basic config information: \n{:?}", config);
        driver.load_config(config);
    }

    // Load AtlasGraph Information.
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(ref atlas_path) = config.atlas {
                let atlas_path = atlas_path.clone();
                if let Err(err) = driver.load(atlas_path).await {
                    eprintln!("{}", err);
                    return;
                }
            }
            if let Err(err) = driver.execute().await {
                eprintln!("{}", err);
            }
        });

    // // generator the ast tree and string.
    // let cypher_ast = driver.construct();
    // // println!("CypherAST:\n{:?}", cypher_ast);

    // // transform
    // let cypher_string = driver.transfrom(Box::new(cypher_ast.clone()));
    // println!("\nCypherString:\n{}", cypher_string);

    // // query number add 1
    // driver.add_query();

    // // log_record recording intermediate information
    // let mut log_record = Log::new();
    // log_record.execute(Box::new(cypher_ast));

    // log_record.report();
}
