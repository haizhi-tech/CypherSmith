use super::{
    cypher::{CypherNode, CypherNodeVisitor},
    expr::{Expression, NodeLabel, VariableGenerator},
};
use crate::common::RandomGenerator;

pub struct GeneratorVisitor {
    random: RandomGenerator,
    variables: VariableGenerator,
}

impl GeneratorVisitor {
    pub fn new() -> Self {
        GeneratorVisitor {
            random: RandomGenerator::new(),
            variables: VariableGenerator::new(),
        }
    }
}

impl GeneratorVisitor {
    pub fn visit(&mut self) -> CypherNode {
        self.visit_query()
    }
}

impl CypherNodeVisitor for GeneratorVisitor {
    type Output = CypherNode;

    fn visit_query(&mut self) -> Self::Output {
        CypherNode::Query {
            query: Box::new(self.visit_regular_query()),
        }
    }

    fn visit_regular_query(&mut self) -> Self::Output {
        CypherNode::RegularQuery {
            single_query: Box::new(self.visit_single_query()),
            union_all: vec![],
        }
    }

    fn visit_single_query(&mut self) -> Self::Output {
        let reading_clause = self.visit_reading_clause();
        let return_clause = self.visit_return();
        CypherNode::SingleQuery {
            reading_clauses: vec![Box::new(reading_clause)],
            updating_clauses: vec![],
            return_clause: Some(Box::new(return_clause)),
        }
    }

    fn visit_reading_clause(&mut self) -> Self::Output {
        let match_pattern = Some(Box::new(self.visit_match()));
        CypherNode::ReadingClause {
            match_clause: match_pattern,
        }
    }

    fn visit_return(&mut self) -> Self::Output {
        // easy case: len(Vec) = 1
        let projection_item = self.visit_projection_item();
        CypherNode::Return {
            projection_body: vec![projection_item],
        }
    }

    fn visit_projection_item(&mut self) -> Self::Output {
        let expression = Expression::new();
        CypherNode::ProjectionItem {
            expressions: vec![(expression, None)],
        }
    }

    fn visit_match(&mut self) -> Self::Output {
        let pattern = Box::new(self.visit_pattern());
        CypherNode::Match {
            is_optional: false,
            pattern,
            where_clause: None,
        }
    }

    fn visit_pattern(&mut self) -> Self::Output {
        let pattern_part = Box::new(self.visit_pattern_part());
        CypherNode::Pattern {
            pattern_parts: vec![pattern_part],
        }
    }

    fn visit_pattern_part(&mut self) -> Self::Output {
        let var = self.variables.new_variable();
        let pattern_element = Box::new(self.visit_pattern_element());
        CypherNode::PatternPart {
            var,
            pattern_element,
        }
    }

    fn visit_pattern_element(&mut self) -> Self::Output {
        let node_pattern = Box::new(self.visit_node_pattern());
        CypherNode::PatternElement {
            pattern_element: vec![(node_pattern, vec![])],
        }
    }

    fn visit_node_pattern(&mut self) -> Self::Output {
        let label = NodeLabel::new();
        CypherNode::NodePattern {
            var: None,
            labels: vec![label],
        }
    }
}
