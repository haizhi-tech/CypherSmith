use super::{
    cypher::{CypherNode, CypherNodeVisitor},
    expr::{Expression, NodeLabel, Variable},
};
use crate::common::RandomGenerator;

struct GeneratorVisitor {
    random: RandomGenerator,
}

impl GeneratorVisitor {
    fn new() -> Self {
        GeneratorVisitor {
            random: RandomGenerator::new(),
        }
    }
}

impl GeneratorVisitor {
    pub fn visit(&self) -> CypherNode {
        self.visit_query()
    }
}

impl CypherNodeVisitor for GeneratorVisitor {
    type Output = CypherNode;

    fn visit_query(&self) -> Self::Output {
        self.visit_regular_query()
    }

    fn visit_regular_query(&self) -> Self::Output {
        self.visit_single_query()
    }

    fn visit_single_query(&self) -> Self::Output {
        let reading_clause = self.visit_reading_clause();
        let return_clause = self.visit_return();
        CypherNode::SingleQuery {
            reading_clauses: vec![Box::new(reading_clause)],
            updating_clauses: vec![],
            return_clause: Some(Box::new(return_clause)),
        }
    }

    fn visit_reading_clause(&self) -> Self::Output {
        let match_pattern = Some(Box::new(self.visit_match()));
        CypherNode::ReadingClause {
            match_clause: match_pattern,
        }
    }

    fn visit_return(&self) -> Self::Output {
        // easy case: len(Vec) = 1
        let projection_item = self.visit_projection_item();
        CypherNode::Return {
            projection_body: vec![projection_item],
        }
    }

    fn visit_projection_item(&self) -> Self::Output {
        let expression = Expression::new();
        CypherNode::ProjectionItem {
            expressions: vec![(expression, None)],
        }
    }

    fn visit_match(&self) -> Self::Output {
        let pattern = Box::new(self.visit_pattern());
        CypherNode::Match {
            is_optional: false,
            pattern,
            where_clause: None,
        }
    }

    fn visit_pattern(&self) -> Self::Output {
        let pattern_part = Box::new(self.visit_pattern_part());
        CypherNode::Pattern {
            pattern_parts: vec![pattern_part],
        }
    }

    fn visit_pattern_part(&self) -> Self::Output {
        let var = Variable::new();
        let pattern_element = Box::new(self.visit_pattern_element());
        CypherNode::PatternPart {
            var,
            pattern_element,
        }
    }

    fn visit_pattern_element(&self) -> Self::Output {
        let node_pattern = Box::new(self.visit_node_pattern());
        CypherNode::PatternElement {
            pattern_element: vec![(node_pattern, vec![])],
        }
    }

    fn visit_node_pattern(&self) -> Self::Output {
        let label = NodeLabel::new();
        CypherNode::NodePattern {
            var: None,
            labels: vec![label],
        }
    }
}
