use cypher_smith::{ArgsConfig, Driver, GraphSchema, Log};

fn main() {
    // get user config.
    let config = <ArgsConfig as clap::Parser>::parse();
    if config.schema.is_none() {
        eprintln!("[WARNING] Please provide schema information!\n\tuse `cypher_smith --help` to find out example usage");
        return;
    }

    // get the label name and so on.
    let mut driver = Driver::new();

    if let Some(ref schema_path) = config.schema {
        let schema_path = schema_path.clone();
        let json = std::fs::read_to_string(schema_path).unwrap();
        let schema = serde_json::from_str::<GraphSchema>(&json).unwrap();
        println!("Input schema information: \n{:?}", schema);
        driver.load_schema(schema);
    }

    // generator the ast tree and string.
    let (cypher_ast, cypher_string) = driver.execute();
    println!("CypherAST:\n{:?}", cypher_ast);
    println!("CypherString:\n{:?}", cypher_string);

    // query number add 1
    driver.add_query();

    // log_record recording intermediate information
    let mut log_record = Log::new();
    log_record.execute(Box::new(cypher_ast));

    log_record.report();
}
