use std::error::Error;

use crate::ast::{CypherNode, LogVisitor};

#[derive(Default)]
pub struct Log {
    queries: u32,
    sum_height: u32,
    sum_nodes: u32,
}

impl Log {
    pub fn new() -> Self {
        Log {
            queries: 0,
            sum_height: 0,
            sum_nodes: 0,
        }
    }

    pub fn stat(&mut self) {
        // self.queries
    }

    // use to get the statistics information.
    // todo:  cypher node need implement copy trait.
    pub fn execute(&mut self, cypher_ast: Box<CypherNode>) {
        let (all_nodes, max_level) = self.visit(cypher_ast);
        self.sum_nodes += all_nodes;
        self.sum_height += max_level;
    }

    // report current cyphersmith condition.
    pub fn report(&self) {
        todo!()
    }
}

impl LogVisitor for Log {
    // todo: need to implement error handle.
    // Output: (max_level, all_nodes)
    type Output = (u32, u32);

    fn visit_query(&mut self, query: Box<CypherNode>) -> Self::Output {
        self.visit(query)
    }

    fn visit_regular_query(
        &mut self,
        single_query: Box<CypherNode>,
        union_all: Vec<Box<CypherNode>>,
    ) -> Self::Output {
        // let (single_query_nodes, single_query_height) = self.visit(single_query);
        // let x = union_all.iter().map(|x| {
            
        // })
        todo!()
    }

    fn visit_standalone_call(
        &mut self,
        procedure: Box<CypherNode>,
        yield_items: (bool, Option<Box<CypherNode>>),
    ) -> Self::Output {
        todo!()
    }

    fn visit_single_query(&mut self, part_query: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_single_part_query(
        &mut self,
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_multi_part_query(
        &mut self,
        multi_part: Vec<(Vec<Box<CypherNode>>, Vec<Box<CypherNode>>, Box<CypherNode>)>,
        single_part: Box<CypherNode>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_with(
        &mut self,
        projection_body: Box<CypherNode>,
        where_clause: Option<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_union(&mut self, union_all: Option<(bool, Box<CypherNode>)>) -> Self::Output {
        todo!()
    }

    fn visit_reading_clause(&mut self, reading_clause: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_updating_clause(&mut self, updating_clause: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_return(&mut self, projection_body: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_projection_body(
        &mut self,
        is_distinct: bool,
        projection_items: Box<CypherNode>,
        order: Option<Box<CypherNode>>,
        skip: Option<super::Expression>,
        limit: Option<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_projection_items(
        &mut self,
        is_all: bool,
        expressions: Vec<(super::Expression, Option<super::Variable>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_order(
        &mut self,
        sort_items: Vec<(super::Expression, Option<String>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_match(
        &mut self,
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_unwind(
        &mut self,
        expression: super::Expression,
        variable: super::Variable,
    ) -> Self::Output {
        todo!()
    }

    fn visit_in_query_call(
        &mut self,
        explicit_proceduce_invocation: Box<CypherNode>,
        yield_items: Option<Box<CypherNode>>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_create(&mut self, pattern: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_merge(
        &mut self,
        pattern_part: Box<CypherNode>,
        merge_actions: Vec<Box<CypherNode>>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_delete(
        &mut self,
        is_detach: bool,
        expressions: Vec<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_set(
        &mut self,
        property_set: Vec<(super::PropertyExpression, super::Expression)>,
        variable_set: Vec<(super::Variable, super::Expression)>,
        variable_add: Vec<(super::Variable, super::Expression)>,
        label_set: Vec<(super::Variable, Vec<super::NodeLabel>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_explicit_procedure_invocation(
        &mut self,
        procedure_name: (super::NameSpace, super::Variable),
        expressions: Vec<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_implicit_procedure_invocation(
        &mut self,
        procedure_name: (super::NameSpace, super::Variable),
    ) -> Self::Output {
        todo!()
    }

    fn visit_yield_items(
        &mut self,
        yield_items: Vec<(Option<super::Variable>, super::Variable)>,
        where_clause: Option<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_remove(
        &mut self,
        variable_remove: Vec<(super::Variable, Vec<super::NodeLabel>)>,
        property_remove: Vec<super::PropertyExpression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_pattern(&mut self, pattern_parts: Vec<Box<CypherNode>>) -> Self::Output {
        todo!()
    }

    fn visit_pattern_part(
        &mut self,
        var: Option<super::Variable>,
        pattern_element: Box<CypherNode>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_pattern_element(
        &mut self,
        parentheses: i32,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    ) -> Self::Output {
        todo!()
    }

    fn visit_node_pattern(
        &mut self,
        var: Option<super::Variable>,
        vertex_labels: Vec<crate::meta::Label>,
        properties: Option<(super::Property, super::FieldValue)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_relationship_pattern(
        &mut self,
        direction: super::RelationshipDirection,
        var: Option<super::Variable>,
        edge_labels: Vec<crate::meta::Label>,
        range: (Option<i32>, Option<i32>),
        properties: Option<(super::Property, super::FieldValue)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_property_or_labels_expression(
        &mut self,
        atom: Box<CypherNode>,
        property_lookups: Vec<super::SchemaName>,
        node_labels: Vec<super::NodeLabel>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_atom(
        &mut self,
        literal: Option<super::Literal>,
        expressions: Vec<super::Expression>,
        sub_expression: Option<Box<CypherNode>>,
        is_variable: Option<super::Variable>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_filter_expression(
        &mut self,
        id_in_coll: (super::Variable, super::Expression),
        where_clause: Option<super::Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_relationships_pattern(
        &mut self,
        node_pattern: Box<CypherNode>,
        pattern_element_chain: Vec<(Box<CypherNode>, Box<CypherNode>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_function_invocation(
        &mut self,
        is_exists: (bool, Option<(super::NameSpace, super::Variable)>),
        is_distinct: bool,
        expressions: Vec<super::Expression>,
    ) -> Self::Output {
        todo!()
    }
}
