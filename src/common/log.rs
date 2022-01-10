use std::cmp;

use super::{Expr, FieldValue, NameSpace, Property, RelationshipDirection, Variable};
use crate::{
    ast::{CypherNode, LogVisitor},
    meta::Label,
};

#[derive(Default)]
pub struct Log {
    // queries: u32,
    sum_height: u32,
    sum_nodes: u32,
}

impl Log {
    pub fn new() -> Self {
        Log {
            // queries: 0,
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
        println!(
            "AST tree information:\nSUM_NODES: {},\nheight: {}",
            self.sum_nodes, self.sum_height
        );
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

    fn get_expr_info(
        &mut self,
        // _expr: Expr,
        sum_nodes: &mut u32,
        max_level: &mut u32,
    ) {
        *sum_nodes += 1;
        *max_level += cmp::max(1, *max_level);
    }

    fn get_single_info(
        &mut self,
        cypher_node: Box<CypherNode>,
        sum_nodes: &mut u32,
        max_level: &mut u32,
    ) {
        let (nodes, height) = self.visit(cypher_node);
        *sum_nodes += nodes;
        *max_level = cmp::max(height, *max_level)
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
        let (mut call_nodes, mut call_height) = self.visit(procedure);
        if let Some(yield_items) = yield_items.1 {
            let (yield_nodes, yield_height) = self.visit(yield_items);
            call_height = cmp::max(call_height, yield_height);
            call_nodes += yield_nodes;
        }

        (call_nodes + 1, call_height + 1)
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
        let (mut multi_nodes, mut multi_height) = (0, 0);

        for (reading_clause, updating_clause, with_clause) in multi_part.into_iter() {
            self.get_info(reading_clause, &mut multi_nodes, &mut multi_height);
            self.get_info(updating_clause, &mut multi_nodes, &mut multi_height);
            self.get_single_info(with_clause, &mut multi_nodes, &mut multi_height);
        }

        self.get_single_info(single_part, &mut multi_nodes, &mut multi_height);

        (multi_nodes + 1, multi_height + 1)
    }

    fn visit_with(
        &mut self,
        projection_body: Box<CypherNode>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        let (mut with_nodes, mut with_height) = (0, 0);

        // process projection_body nodes
        self.get_single_info(projection_body, &mut with_nodes, &mut with_height);

        // todo: expr process.
        if where_clause.is_some() {}

        (with_nodes + 1, with_height + 1)
    }

    fn visit_union(&mut self, union_all: Option<(bool, Box<CypherNode>)>) -> Self::Output {
        let (mut union_nodes, mut union_height) = (0, 0);

        if let Some((_, union_all_node)) = union_all {
            self.get_single_info(union_all_node, &mut union_nodes, &mut union_height);
        }

        (union_nodes, union_height)
    }

    fn visit_reading_clause(&mut self, reading_clause: Box<CypherNode>) -> Self::Output {
        self.visit(reading_clause)
    }

    fn visit_updating_clause(&mut self, updating_clause: Box<CypherNode>) -> Self::Output {
        self.visit(updating_clause)
    }

    fn visit_return(&mut self, projection_body: Box<CypherNode>) -> Self::Output {
        self.visit(projection_body)
    }

    fn visit_projection_body(
        &mut self,
        _is_distinct: bool,
        projection_items: Box<CypherNode>,
        order: Option<Box<CypherNode>>,
        skip: Option<Expr>,
        limit: Option<Expr>,
    ) -> Self::Output {
        let (mut nodes, mut height) = (0, 0);

        self.get_single_info(projection_items, &mut nodes, &mut height);

        if let Some(order_node) = order {
            self.get_single_info(order_node, &mut nodes, &mut height);
        }

        if skip.is_some() {
            self.get_expr_info(&mut nodes, &mut height);
        }

        if limit.is_some() {
            self.get_expr_info(&mut nodes, &mut height);
        }

        (nodes, height)
    }

    fn visit_projection_items(
        &mut self,
        _is_all: bool,
        expressions: Vec<(Expr, Option<Variable>)>,
    ) -> Self::Output {
        (expressions.len() as u32, 1)
    }

    fn visit_order(&mut self, sort_items: Vec<(Expr, Option<String>)>) -> Self::Output {
        (sort_items.len() as u32, 1)
    }

    fn visit_match(
        &mut self,
        _is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        let (mut pattern_nodes, pattern_height) = self.visit(pattern);

        // where clause.
        if where_clause.is_some() {
            pattern_nodes += 1;
        }

        (pattern_nodes + 1, pattern_height + 1)
    }

    fn visit_unwind(&mut self, _expression: Expr, _variable: Variable) -> Self::Output {
        (1, 1)
    }

    fn visit_in_query_call(
        &mut self,
        explicit_proceduce_invocation: Box<CypherNode>,
        yield_items: Option<Box<CypherNode>>,
    ) -> Self::Output {
        let (mut call_nodes, mut call_height) = self.visit(explicit_proceduce_invocation);

        if let Some(yield_items) = yield_items {
            self.get_single_info(yield_items, &mut call_nodes, &mut call_height);
        }

        (call_nodes + 1, call_height)
    }

    fn visit_create(&mut self, pattern: Box<CypherNode>) -> Self::Output {
        self.visit(pattern)
    }

    fn visit_merge(
        &mut self,
        pattern_part: Box<CypherNode>,
        merge_actions: Vec<(String, Box<CypherNode>)>,
    ) -> Self::Output {
        let (mut merge_nodes, mut merge_height) = self.visit(pattern_part);

        let merge_action_nodes = merge_actions.into_iter().map(|x| x.1).collect::<Vec<_>>();
        self.get_info(merge_action_nodes, &mut merge_nodes, &mut merge_height);

        (merge_nodes, merge_height)
    }

    fn visit_delete(&mut self, _is_detach: bool, expressions: Vec<Expr>) -> Self::Output {
        (expressions.len() as u32, 1)
    }

    fn visit_set(
        &mut self,
        property_set: Vec<(Expr, Expr)>,
        variable_set: Vec<(Variable, Expr)>,
        variable_add: Vec<(Variable, Expr)>,
        label_set: Vec<(Variable, Vec<Label>)>,
    ) -> Self::Output {
        let sum_nodes =
            property_set.len() + variable_set.len() + variable_add.len() + label_set.len();
        (sum_nodes as u32, 1)
    }

    fn visit_explicit_procedure_invocation(
        &mut self,
        _procedure_name: (NameSpace, Variable),
        expressions: Vec<Expr>,
    ) -> Self::Output {
        (expressions.len() as u32, 1)
    }

    fn visit_implicit_procedure_invocation(
        &mut self,
        _procedure_name: (NameSpace, Variable),
    ) -> Self::Output {
        (1, 1)
    }

    fn visit_yield_items(
        &mut self,
        yield_items: Vec<(Option<Variable>, Variable)>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        let mut yield_items_nodes = yield_items.len();

        if where_clause.is_some() {
            yield_items_nodes += 1;
        }

        (yield_items_nodes as u32, 1)
    }

    fn visit_remove(
        &mut self,
        variable_remove: Vec<(Variable, Vec<Label>)>,
        property_remove: Vec<Expr>,
    ) -> Self::Output {
        let sum_nodes = variable_remove.len() + property_remove.len();
        (sum_nodes as u32, 1)
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
        _parentheses: bool,
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
        _direction: RelationshipDirection,
        _var: Option<Variable>,
        _edge_labels: Vec<crate::meta::Label>,
        _is_range: bool,
        _range: (Option<i32>, Option<(bool, Option<i32>)>),
        _properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        (1, 1)
    }
}
