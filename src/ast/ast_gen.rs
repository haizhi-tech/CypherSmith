use super::{
    cypher::{CypherNode, CypherNodeVisitor},
    expr::{Expression, NodeLabel, VariableGenerator},
};
use crate::common::RandomGenerator;

pub struct CypherGenerator {
    query_string: String,
    random: RandomGenerator,
    variables: VariableGenerator,
}

impl CypherGenerator {
    pub fn new() -> Self {
        CypherGenerator {
            query_string: String::new(),
            random: RandomGenerator::new(),
            variables: VariableGenerator::new(),
        }
    }
}

impl CypherGenerator {
    pub fn visit(&mut self) -> CypherNode {
        self.visit_query()
    }
}

impl CypherNodeVisitor for CypherGenerator {
    type Output = CypherNode;

    /// query: regular_query | standaloneCall
    fn visit_query(&mut self) -> Self::Output {
        let ty =self.random.d2();
        let query = if ty != 0 {
            self.visit_regular_query()
        }  else {
            // self.visit_standalone_call()
            todo!()
        };
        CypherNode::Query {
            query: Box::new(query),
        }
    }

    fn visit_regular_query(&mut self) -> Self::Output {
        let union_number = self.random.d2();
        let mut union_all = vec![];
        for _ in 0..union_number {
            union_all.push(Box::new(self.visit_union()));
        }
        CypherNode::RegularQuery {
            single_query: Box::new(self.visit_single_query()),
            union_all,
        }
    }

    fn visit_standalone_call(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_union(&mut self) -> Self::Output {
        let union_type = self.random.d2();
        let sub_query = Box::new(self.visit_single_query());
        let is_all = if union_type > 0 {
            true
        } else {
            false
        };

        CypherNode::Union {
            union_all: Some((is_all, sub_query)),
        }
    }

    fn visit_single_query(&mut self) -> Self::Output {
        let query = if self.random.d2() > 0 {
            self.visit_single_part_query()
        } else {
            self.visit_multi_part_query()
        };

        CypherNode::SingleQuery {
            part_query: Box::new(query),
        }
    }


    /// SinglePartQuery: ReadingClause* Return | ReadingClause* UpdatingClause+ Return?
    fn visit_single_part_query(&mut self) -> Self::Output {
        if self.random.d2() > 0 {
            let reading_number = self.random.d2();
            let mut reading_clauses = vec![];
            for _ in 0..reading_number {
                reading_clauses.push(Box::new(self.visit_reading_clause()));
            }

            let return_clause = self.visit_return();
            CypherNode::SinglePartQuery {
                reading_clauses,
                updating_clauses: vec![],
                return_clause: Some(Box::new(return_clause)),
            }
        } else {
            let mut reading_clauses = vec![];
            let mut updating_clauses = vec![];
            for _ in 0..self.random.d2() {
                reading_clauses.push(Box::new(self.visit_reading_clause()));
            }
            updating_clauses.push(Box::new(self.visit_updating_clause()));
            for _ in 0..self.random.d2() {
                updating_clauses.push(Box::new(self.visit_updating_clause()));
            }
            let return_clause = if self.random.d2() > 0 {
                Some(Box::new(self.visit_return()))
            } else {
                None
            };
            CypherNode::SinglePartQuery {
                reading_clauses,
                updating_clauses,
                return_clause,
            }
        }
    }

    // multi_part: ((ReadingClause)* (Updating_clause)* With)+ SinglePartQuery
    fn visit_multi_part_query(&mut self) -> Self::Output {
        let single_part = Box::new(self.visit_single_part_query());
        let mut multi_part = vec![];

        let with_number = self.random.d2();
        for _ in 0..with_number {
            let reading_clause = vec![];
            let updating_clause = vec![];
            let reading_number = self.random.d2();
            let updating_number = self.random.d2();

            for _ in  0..reading_number {
                reading_clause.push(Box::new(self.visit_reading_clause()));
            }

            for _ in 0..updating_number {
                updating_clause.push(Box::new(self.visit_return()));
            }

            let with_query = Box::new(self.visit_with());
            multi_part.push((reading_clause, updating_clause, with_query));
        }

        CypherNode::MultiPartQuery {
            multi_part,
            single_part,
        }
    }

    fn visit_with(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_reading_clause(&mut self) -> Self::Output {
        let reading_clause = match self.random.d6() {
            0 => self.visit_match(),
            1 => self.visit_unwind(),
            2 => self.visit_in_query_call(),
            _ => {
                // todo: have not implement
                self.visit_match()
            },
        };
        
        CypherNode::ReadingClause {
            reading_clause: Box::new(reading_clause),
        }
    }

    fn visit_updating_clause(&mut self) -> Self::Output {
        todo!()
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

    // todo
    fn visit_unwind(&mut self) -> Self::Output {
        todo!()
    }

    // todo!()
    fn visit_in_query_call(&mut self) ->Self::Output {
        todo!()
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
