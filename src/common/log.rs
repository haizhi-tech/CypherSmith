use std::{cmp, error::Error};

use super::{
    Expression, FieldValue, Literal, NameSpace, NodeLabel, Property, PropertyExpression,
    RelationshipDirection, SchemaName, Variable,
};
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

    fn get_info(
        &mut self,
        cypher_nodes: Vec<Box<CypherNode>>,
        sum_nodes: &mut u32,
        max_level: &mut u32,
    ) {
        for node in cypher_nodes {
            let (nodes, height) = self.visit(node);
            *sum_nodes += nodes;
            *max_level = cmp::max(height, *max_level)
        }
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
        let (mut regular_nodes, mut regular_height) = self.visit(single_query);
        self.get_info(union_all, &mut regular_nodes, &mut regular_height);

        (regular_nodes + 1, regular_height + 1)
    }

    fn visit_standalone_call(
        &mut self,
        procedure: Box<CypherNode>,
        yield_items: (bool, Option<Box<CypherNode>>),
    ) -> Self::Output {
        todo!()
    }

    fn visit_single_query(&mut self, part_query: Box<CypherNode>) -> Self::Output {
        self.visit(part_query)
    }

    fn visit_single_part_query(
        &mut self,
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    ) -> Self::Output {
        let (mut single_part_nodes, mut single_part_height) = (0, 0);

        self.get_info(
            reading_clauses,
            &mut single_part_nodes,
            &mut single_part_height,
        );
        self.get_info(
            updating_clauses,
            &mut single_part_nodes,
            &mut single_part_height,
        );

        if let Some(return_clause) = return_clause {
            let (nodes, height) = self.visit(return_clause);
            single_part_nodes += nodes;
            single_part_height = cmp::max(height, single_part_height);
        }

        (single_part_nodes + 1, single_part_height + 1)
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
        where_clause: Option<Expression>,
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
        self.visit(updating_clause)
    }

    fn visit_return(&mut self, projection_body: Box<CypherNode>) -> Self::Output {
        todo!()
    }

    fn visit_projection_body(
        &mut self,
        is_distinct: bool,
        projection_items: Box<CypherNode>,
        order: Option<Box<CypherNode>>,
        skip: Option<Expression>,
        limit: Option<Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_projection_items(
        &mut self,
        is_all: bool,
        expressions: Vec<(Expression, Option<Variable>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_order(&mut self, sort_items: Vec<(Expression, Option<String>)>) -> Self::Output {
        todo!()
    }

    fn visit_match(
        &mut self,
        _is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Expression>,
    ) -> Self::Output {
        let (pattern_nodes, pattern_height) = self.visit(pattern);
        (pattern_nodes + 1, pattern_height + 1)
    }

    fn visit_unwind(&mut self, expression: Expression, variable: Variable) -> Self::Output {
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

    fn visit_delete(&mut self, is_detach: bool, expressions: Vec<Expression>) -> Self::Output {
        todo!()
    }

    fn visit_set(
        &mut self,
        property_set: Vec<(PropertyExpression, Expression)>,
        variable_set: Vec<(Variable, Expression)>,
        variable_add: Vec<(Variable, Expression)>,
        label_set: Vec<(Variable, Vec<NodeLabel>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_explicit_procedure_invocation(
        &mut self,
        procedure_name: (NameSpace, Variable),
        expressions: Vec<Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_implicit_procedure_invocation(
        &mut self,
        procedure_name: (NameSpace, Variable),
    ) -> Self::Output {
        todo!()
    }

    fn visit_yield_items(
        &mut self,
        yield_items: Vec<(Option<Variable>, Variable)>,
        where_clause: Option<Expression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_remove(
        &mut self,
        variable_remove: Vec<(Variable, Vec<NodeLabel>)>,
        property_remove: Vec<PropertyExpression>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_pattern(&mut self, pattern_parts: Vec<Box<CypherNode>>) -> Self::Output {
        let (mut pattern_parts_nodes, mut pattern_parts_height) = (0, 0);
        self.get_info(
            pattern_parts,
            &mut pattern_parts_nodes,
            &mut pattern_parts_height,
        );
        (pattern_parts_nodes, pattern_parts_height)
    }

    fn visit_pattern_part(
        &mut self,
        _var: Option<Variable>,
        pattern_element: Box<CypherNode>,
    ) -> Self::Output {
        self.visit(pattern_element)
    }

    fn visit_pattern_element(
        &mut self,
        _parentheses: i32,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    ) -> Self::Output {
        let (mut nodes, mut height) = self.visit(pattern_element.0);
        for (first, second) in pattern_element.1 {
            let (nodes_first, height_first) = self.visit(first);
            let (nodes_second, height_second) = self.visit(second);
            nodes += nodes_first;
            nodes += nodes_second;
            height = cmp::max(height, height_first);
            height = cmp::max(height, height_second);
        }
        (nodes, height)
    }

    fn visit_node_pattern(
        &mut self,
        _var: Option<Variable>,
        _vertex_labels: Vec<crate::meta::Label>,
        _properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        (1, 1)
    }

    fn visit_relationship_pattern(
        &mut self,
        direction: RelationshipDirection,
        var: Option<Variable>,
        edge_labels: Vec<crate::meta::Label>,
        range: (Option<i32>, Option<i32>),
        properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_property_or_labels_expression(
        &mut self,
        atom: Box<CypherNode>,
        property_lookups: Vec<SchemaName>,
        node_labels: Vec<NodeLabel>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_atom(
        &mut self,
        literal: Option<Literal>,
        expressions: Vec<Expression>,
        sub_expression: Option<Box<CypherNode>>,
        is_variable: Option<Variable>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_filter_expression(
        &mut self,
        id_in_coll: (Variable, Expression),
        where_clause: Option<Expression>,
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
        is_exists: (bool, Option<(NameSpace, Variable)>),
        is_distinct: bool,
        expressions: Vec<Expression>,
    ) -> Self::Output {
        todo!()
    }
}
