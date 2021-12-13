use cypher_smith::{Driver, Log};

fn main() {
    // add connection and get the schema information.
    // get the label name and so on.
    let mut driver = Driver::new();
    let cypher_ast = driver.execute();

    println!("{:?}", cypher_ast);
    let cypher_string = driver.transfrom(Box::new(cypher_ast));
    driver.add_query();

    // let logger = Log::new();
    // logger.execute(&cypher_ast);

    // logger.report();
    println!("{}", cypher_string);
}
