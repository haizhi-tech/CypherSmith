use super::{
    cypher::{ConvertVisitor, CypherNode},
    expr::{Expression, NodeLabel, Variable},
};

pub struct TransformVisitor {
    cypher_string: String,
}

impl TransformVisitor {
    fn new() -> TransformVisitor {
        Self {
            cypher_string: "".to_string(),
        }
    }
}

impl ConvertVisitor for TransformVisitor {
    type Output = String;

    fn visit_query(&mut self, query: Box<CypherNode>) -> Self::Output {
        let ret = match *query {
            CypherNode::RegularQuery {
                single_query,
                union_all,
            } => self.visit_regular_query(single_query, union_all),
            _ => todo!(),
        };
        ret
    }

    fn visit_regular_query(
        &mut self,
        single_query: Box<CypherNode>,
        _union_all: Vec<Box<CypherNode>>,
    ) -> Self::Output {
        match *single_query {
            CypherNode::SingleQuery {
                reading_clauses,
                updating_clauses,
                return_clause,
            } => self.visit_single_query(reading_clauses, updating_clauses, return_clause),
            _ => todo!(),
        }
    }

    fn visit_single_query(
        &mut self,
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    ) -> Self::Output {
        // let reading_clause = reading_clauses.get(0).unwrap();
        // let (reading_str, return_str) = match (
        //     **reading_clause,
        //     updating_clauses,
        //     *return_clause.unwrap(),
        // ) {
        //     (
        //         CypherNode::ReadingClause { match_clause },
        //         _,
        //         CypherNode::Return { projection_body },
        //     ) => (
        //         self.visit_reading_clause(match_clause),
        //         self.visit_return(projection_body),
        //     ),
        //     _ => todo!(),
        // };
        // let ret = reading_str + &return_str;
        // ret
        todo!()
    }

    fn visit_reading_clause(&mut self, match_clause: Option<Box<CypherNode>>) -> Self::Output {
        if let Some(match_clause) = match_clause {
            match *match_clause {
                CypherNode::Match {
                    is_optional,
                    pattern,
                    where_clause,
                } => self.visit_match(is_optional, pattern, where_clause),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    fn visit_return(&mut self, projection_body: Vec<CypherNode>) -> Self::Output {
        let mut ret_string = "".to_string();
        for iter in projection_body {
            let iter_str = match iter {
                CypherNode::ProjectionItem { expressions } => {
                    self.visit_projection_item(expressions)
                }
                _ => todo!(),
            };
            ret_string += &iter_str;
        }
        ret_string
    }

    fn visit_projection_item(
        &mut self,
        expressions: Vec<(Expression, Option<Variable>)>,
    ) -> Self::Output {
        // let (expression, var) = expressions[0];
        // expression.get_name()
        todo!()
    }

    fn visit_match(
        &mut self,
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Box<CypherNode>>,
    ) -> Self::Output {
        // easy case
        match *pattern {
            CypherNode::Pattern { pattern_parts } => self.visit_pattern(pattern_parts),
            _ => todo!(),
        }
    }

    fn visit_pattern(&mut self, pattern_parts: Vec<Box<CypherNode>>) -> Self::Output {
        // if pattern_parts.len() == 1 {
        //     let pattern_part = pattern_parts[0];
        //     match *pattern_part {
        //         CypherNode::PatternPart {
        //             var,
        //             pattern_element,
        //         } => self.visit_pattern_part(var, pattern_element),
        //         _ => todo!(),
        //     }
        // } else {
        //     todo!()
        // }
        todo!()
    }

    fn visit_pattern_part(
        &mut self,
        var: Variable,
        pattern_element: Box<CypherNode>,
    ) -> Self::Output {
        let var_str = var.get_name();
        let pattern_string = match *pattern_element {
            CypherNode::PatternElement { pattern_element } => {
                self.visit_pattern_element(pattern_element)
            }
            _ => todo!(),
        };
        let ret = var_str + &pattern_string;
        ret
    }

    fn visit_pattern_element(
        &mut self,
        pattern_element: Vec<(Box<CypherNode>, Vec<Box<CypherNode>>)>,
    ) -> Self::Output {
        // if pattern_element.len() == 1 {
        //     let (first, _second) = pattern_element[0];
        //      match *first {
        //         CypherNode::NodePattern { var, labels } => {
        //             self.visit_node_pattern(var, labels)
        //         }
        //         _ => todo!(),
        //     }
        // } else {
        //     todo!()
        // }
        todo!()
    }

    fn visit_node_pattern(
        &mut self,
        var: Option<Variable>,
        labels: Vec<NodeLabel>,
    ) -> Self::Output {
        let (var_str, label_str) = if var.is_some() || labels.len() != 1 {
            todo!()
        } else {
            // TODO: NodeLabel Display
            ("".to_string(), "".to_string())
        };
        let ret = var_str + &label_str;
        ret
    }
}
