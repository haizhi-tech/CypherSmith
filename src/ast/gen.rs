use super::cypher::{CypherNode, CypherNodeVisitor};

struct GeneratorVisitor {}

impl CypherNodeVisitor for GeneratorVisitor {
    type Output = CypherNode;

    fn visit_query(&mut self, query: &Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_regular_query(
        &mut self,
        single_query: &Box<CypherNode>,
        union_all: &Vec<Box<CypherNode>>,
    ) -> Self::Output {
        todo!()
    }
}
