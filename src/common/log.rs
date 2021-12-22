use crate::ast::CypherNode;

trait BasicLog {}

#[derive(Default)]
pub struct Log {
    queries: u32,
    height: u32,
    nodes: u32,
}

impl BasicLog for Log {}

impl Log {
    pub fn new() -> Self {
        Log {
            queries: 0,
            height: 0,
            nodes: 0,
        }
    }

    // use to get the statistics information.
    pub fn execute(&self, _cypher_ast: &CypherNode) {
        todo!()
    }

    // report current cyphersmith condition.
    pub fn report(&self) {
        todo!()
    }
}
