use cypher_smith::{Driver, Log, CliArgsConfig, GraphSchema};

fn main() {
    // add connection and get the schema information.
    let config = <CliArgsConfig as clap::Parser>::parse();

    if config.schema.is_none() {
        eprintln!("no args input, do nothing!\nuse `graph_importer --help` to find out example usage");
    }

    // get the label name and so on.
    let mut driver = Driver::new();

    if let Some(ref schema_path) = config.schema {
        let schema_path = schema_path.clone();
        let json = std::fs::read_to_string(schema_path).unwrap();
        let schema = serde_json::from_str::<GraphSchema>(&json).unwrap();
        driver.load_schema(schema);
    }

    // generator the ast tree and string.
    let (cypher_ast, cypher_string) = driver.execute();
    println!("{:?}", cypher_ast);
    
    // query number add 1
    driver.add_query();

    // logger recording intermediate information
    let logger = Log::new();
    logger.execute(&cypher_ast);

    logger.report();
    println!("{}", cypher_string);
}
