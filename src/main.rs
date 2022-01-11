use cypher_smith::{ArgsConfig, Driver, Log};

fn main() {
    // get user config.
    let config = <ArgsConfig as clap::Parser>::parse();

    if config.schema.is_none() {
        eprintln!("[WARNING] Please provide schema information!\n\tuse `cypher_smith --help` to find out example usage");
        return;
    }

    // get the label name and so on.
    let mut driver = Driver::new();

    // load information.
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(err) = driver.load(config).await {
                eprintln!("{}", err);
            }
        });

    // generator the ast tree and string.
    let cypher_ast = driver.construct();
    // println!("CypherAST:\n{:?}", cypher_ast);

    // transform
    let cypher_string = driver.transfrom(Box::new(cypher_ast.clone()));
    println!("\nCypherString:\n{}", cypher_string);

    // query number add 1
    driver.add_query();

    // log_record recording intermediate information
    let mut log_record = Log::new();
    log_record.execute(Box::new(cypher_ast));

    log_record.report();
}
