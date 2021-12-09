fn main() {
    // add connection and get the schema information.
    // get the label name and so on.
    let mut driver = cypher_smith::Driver::new();
    let cypher_ast = driver.execute();
    let cypher_string = driver.transfrom(&cypher_ast);
    driver.add_query();

    let logger = cypher_smith::Log::new();
    logger.execute(&cypher_ast);

    logger.report();
    println!("{}", cypher_string);
}
