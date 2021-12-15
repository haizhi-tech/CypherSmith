use super::{
    cypher::{CypherNode, CypherNodeVisitor},
    expr::{Expression, NodeLabel, Properties, RelationshipDirection, VariableGenerator},
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
        let ty = self.random.d2();
        let query = if ty != 0 {
            self.visit_regular_query()
        } else {
            self.visit_standalone_call()
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

    // todo: need to implementation.
    fn visit_standalone_call(&mut self) -> Self::Output {
        self.visit_regular_query()
    }

    fn visit_union(&mut self) -> Self::Output {
        let sub_query = Box::new(self.visit_single_query());

        CypherNode::Union {
            union_all: Some((self.random.bool(), sub_query)),
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
            let mut reading_clause = vec![];
            let mut updating_clause = vec![];
            let reading_number = self.random.d2();
            let updating_number = self.random.d2();

            for _ in 0..reading_number {
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
        let projection_body = Box::new(self.visit_projection_body());
        let where_clause = if self.random.bool() {
            Some(Expression::new())
        } else {
            None
        };
        CypherNode::With {
            projection_body,
            where_clause,
        }
    }

    fn visit_reading_clause(&mut self) -> Self::Output {
        let reading_clause = match self.random.d6() {
            0 => self.visit_match(),
            1 => self.visit_unwind(),
            2 => self.visit_in_query_call(),
            _ => {
                // todo: need to modify
                self.visit_match()
            }
        };

        CypherNode::ReadingClause {
            reading_clause: Box::new(reading_clause),
        }
    }

    fn visit_match(&mut self) -> Self::Output {
        let pattern = Box::new(self.visit_pattern());
        let where_clause = if self.random.bool() {
            Some(Expression::new())
        } else {
            None
        };
        CypherNode::Match {
            is_optional: self.random.bool(),
            pattern,
            where_clause,
        }
    }

    // unwind: UNWIND expression AS variable.
    fn visit_unwind(&mut self) -> Self::Output {
        CypherNode::Unwind {
            expression: Expression::new(),
            variable: self.variables.new_variable(),
        }
    }

    // todo: need implementation.
    fn visit_in_query_call(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_updating_clause(&mut self) -> Self::Output {
        let updating_clause = match self.random.d6() {
            0 => self.visit_create(),
            1 => self.visit_merge(),
            2 => self.visit_delete(),
            3 => self.visit_set(),
            4 => self.visit_remove(),
            _ => {
                // todo: need to modify
                self.visit_create()
            }
        };

        CypherNode::UpdatingClause {
            updating_clause: Box::new(updating_clause),
        }
    }

    fn visit_create(&mut self) -> Self::Output {
        CypherNode::Create {
            pattern: Box::new(self.visit_pattern()),
        }
    }

    // todo
    fn visit_merge(&mut self) -> Self::Output {
        todo!()
    }

    // todo
    fn visit_delete(&mut self) -> Self::Output {
        todo!()
    }

    // todo
    fn visit_set(&mut self) -> Self::Output {
        todo!()
    }

    // todo
    fn visit_remove(&mut self) -> Self::Output {
        todo!()
    }

    /// Return clause: return projection_body.
    fn visit_return(&mut self) -> Self::Output {
        let projection_body = Box::new(self.visit_projection_body());
        CypherNode::Return { projection_body }
    }

    fn visit_projection_body(&mut self) -> Self::Output {
        let is_distinct = self.random.bool();
        let projection_items = Box::new(self.visit_projection_items());
        // order:
        let order = if self.random.low_prob_bool() {
            Some(Box::new(self.visit_order()))
        } else {
            None
        };
        let skip = if self.random.low_prob_bool() {
            Some(Expression::new())
        } else {
            None
        };
        let limit = if self.random.low_prob_bool() {
            Some(Expression::new())
        } else {
            None
        };
        CypherNode::ProjectionBody {
            is_distinct,
            projection_items,
            order,
            skip,
            limit,
        }
    }

    fn visit_projection_items(&mut self) -> Self::Output {
        let mut expressions = Vec::new();
        let is_all = if self.random.bool() {
            true
        } else {
            let var = if self.random.bool() {
                Some(self.variables.new_variable())
            } else {
                None
            };
            expressions.push((Expression::new(), var));
            false
        };

        // projection_items
        for _ in 0..self.random.d2() {
            let var = if self.random.bool() {
                Some(self.variables.new_variable())
            } else {
                None
            };
            expressions.push((Expression::new(), var));
        }

        CypherNode::ProjectionItems {
            is_all,
            expressions,
        }
    }

    /// order: order by sort_items
    fn visit_order(&mut self) -> Self::Output {
        let sort_rules = vec!["ASC", "DESC", "ASCENDING", "DESCENDING"];
        let mut sort_items = vec![];
        let rule = if self.random.bool() {
            Some(sort_rules[self.random.d2() as usize].to_string())
        } else {
            None
        };
        sort_items.push((Expression::new(), rule));

        for _ in 0..self.random.d2() {
            let rule = if self.random.bool() {
                Some(sort_rules[self.random.d2() as usize].to_string())
            } else {
                None
            };
            sort_items.push((Expression::new(), rule))
        }

        CypherNode::Order { sort_items }
    }

    // Pattern: PatternPart*
    fn visit_pattern(&mut self) -> Self::Output {
        let mut pattern_parts = vec![];
        for _ in 0..self.random.d2() {
            pattern_parts.push(Box::new(self.visit_pattern_part()));
        }

        CypherNode::Pattern { pattern_parts }
    }

    // PatternPart: (Variable =)? pattern_element
    fn visit_pattern_part(&mut self) -> Self::Output {
        let var = if self.random.bool() {
            Some(self.variables.new_variable())
        } else {
            None
        };

        let pattern_element = Box::new(self.visit_pattern_element());
        CypherNode::PatternPart {
            var,
            pattern_element,
        }
    }

    // pattern_element: NodePattern (RelationshipPattern NodePattern)*
    fn visit_pattern_element(&mut self) -> Self::Output {
        let node_pattern = Box::new(self.visit_node_pattern());

        let mut pattern_element_chain = vec![];
        for _ in 0..self.random.d2() {
            pattern_element_chain.push((
                Box::new(self.visit_relationship_pattern()),
                Box::new(self.visit_node_pattern()),
            ));
        }

        CypherNode::PatternElement {
            parentheses: self.random.d2(),
            pattern_element: (node_pattern, pattern_element_chain),
        }
    }

    // NodePattern: ( Variable? (:label)* Properties)
    fn visit_node_pattern(&mut self) -> Self::Output {
        let var = if self.random.bool() {
            Some(self.variables.new_variable())
        } else {
            None
        };

        let mut vertex_labels = vec![];
        for _ in 0..self.random.d2() {
            vertex_labels.push(NodeLabel::new());
        }

        let properties = if self.random.bool() {
            Some(Properties::new())
        } else {
            None
        };

        CypherNode::NodePattern {
            var,
            vertex_labels,
            properties,
        }
    }

    fn visit_relationship_pattern(&mut self) -> Self::Output {
        let direction = match self.random.d6() {
            0 => RelationshipDirection::Left,
            1 => RelationshipDirection::Right,
            2 => RelationshipDirection::Both,
            3 => RelationshipDirection::None,
            _ => RelationshipDirection::None,
        };
        let var = if self.random.bool() {
            Some(self.variables.new_variable())
        } else {
            None
        };
        let mut edge_labels = vec![];
        for _ in 0..self.random.d2() {
            edge_labels.push(NodeLabel::new());
        }
        let range_start = self.random.d2();
        let range_end = self.random.d6();

        let properties = if self.random.bool() {
            Some(Properties::new())
        } else {
            None
        };

        CypherNode::RelationshipPattern {
            direction,
            var,
            edge_labels,
            range_start,
            range_end,
            properties,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::CypherGenerator;

    #[test]
    fn query_test() {
        let mut generator = CypherGenerator::new();
        println!("{:?}", generator.visit());
    }
}
