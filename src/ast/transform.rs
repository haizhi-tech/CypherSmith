use super::cypher::{CypherNode, CypherNodeVisitor};

pub struct TransformVisitor {}

impl TransformVisitor {
    fn new() -> TransformVisitor {
        Self {}
    }
}

impl CypherNodeVisitor for TransformVisitor {
    type Output = String;

    fn visit_query(&mut self, _query: &Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_regular_query(
        &mut self,
        _single_query: &Box<CypherNode>,
        _union_all: &Vec<Box<CypherNode>>,
    ) -> Self::Output {
        todo!()
    }
}
