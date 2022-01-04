use super::cypher::{ConvertVisitor, CypherNode};

use crate::common::{Expr, FieldValue, NodeLabel, Property, Variable};
use crate::meta::Label;

pub struct TransformVisitor {
    cypher_string: String,
}

impl TransformVisitor {
    pub fn new() -> TransformVisitor {
        Self {
            cypher_string: "".to_string(),
        }
    }

    pub fn exec(&mut self, query: Box<CypherNode>) -> String {
        // self.visit(query)
        todo!()
    }
}

impl ConvertVisitor for TransformVisitor {
    type Output = String;

    /// Query: RegularQuery | StandaloneCall
    fn visit_query(&mut self, query: Box<CypherNode>) -> Self::Output {
        // visit: match node => transform appropriate node type.
        self.visit(query)
    }

    /// RegulayQuery: SingleQuery {Union}*
    fn visit_regular_query(
        &mut self,
        single_query: Box<CypherNode>,
        union_all: Vec<Box<CypherNode>>,
    ) -> Self::Output {
        let mut regular_string = self.visit(single_query);

        for union_node in union_all {
            let union_string = self.visit(union_node);
            regular_string += " ";
            regular_string += &union_string;
        }

        regular_string
    }

    fn visit_standalone_call(
        &mut self,
        procedure: Box<CypherNode>,
        yield_items: (bool, Option<Box<CypherNode>>),
    ) -> Self::Output {
        todo!()
    }

    /// SinglePartQuery: SinglePartQuery | MultiPartQuery.
    fn visit_single_query(&mut self, part_query: Box<CypherNode>) -> Self::Output {
        self.visit(part_query)
    }

    ///  SinglePartQuery: ReadingClause* Return | ReadingClause* UpdatingClause+ Return?
    fn visit_single_part_query(
        &mut self,
        reading_clauses: Vec<Box<CypherNode>>,
        updating_clauses: Vec<Box<CypherNode>>,
        return_clause: Option<Box<CypherNode>>,
    ) -> Self::Output {
        let mut query_string = String::new();

        for reading_clause in reading_clauses {
            query_string += &self.visit(reading_clause);
            query_string += " ";
        }

        for updating_clause in updating_clauses {
            query_string += &self.visit_updating_clause(updating_clause);
            query_string += " ";
        }

        if let Some(return_clause) = return_clause {
            query_string += " ";
            query_string += &self.visit(return_clause);
        }

        query_string
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
        where_clause: Option<Expr>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_union(&mut self, union_all: Option<(bool, Box<CypherNode>)>) -> Self::Output {
        todo!()
    }

    /// ReadingClause: Match|Unwind|InqueryCall
    fn visit_reading_clause(&mut self, reading_clause: Box<CypherNode>) -> Self::Output {
        self.visit(reading_clause)
    }

    /// UpdatingClause: Create|Merge|Delete|Set|Remove
    fn visit_updating_clause(&mut self, updating_clause: Box<CypherNode>) -> Self::Output {
        self.visit(updating_clause)
    }

    /// Return: `return` ProjectionBody
    fn visit_return(&mut self, projection_body: Box<CypherNode>) -> Self::Output {
        let mut query_string = "RETURN".to_string();
        query_string += &self.visit(projection_body);

        query_string
    }

    /// ProjectionBody: `[DISTINCT]` ProjectionItems `[Order]` `[Skip]` `[Limit]`
    fn visit_projection_body(
        &mut self,
        is_distinct: bool,
        projection_items: Box<CypherNode>,
        order: Option<Box<CypherNode>>,
        skip: Option<Expr>,
        limit: Option<Expr>,
    ) -> Self::Output {
        let mut query_string = String::new();

        if is_distinct {
            query_string += " DISTINCT";
        }

        query_string += " ";
        query_string += &self.visit(projection_items);

        // todo: Expr to string, how to transform.
        if let Some(order_node) = order {
            query_string += " ";
            query_string += &self.visit(order_node);
        }
        if let Some(skip_node) = skip {
            todo!()
        }
        if let Some(limit_node) = limit {
            todo!()
        }

        query_string
    }

    /// ProjectionItems: *(,ProjectionItem)*|ProjectionItem+
    fn visit_projection_items(
        &mut self,
        is_all: bool,
        expressions: Vec<(Expr, Option<crate::common::Variable>)>,
    ) -> Self::Output {
        let mut query_string = String::new();

        if is_all {
            // is_all = true: *
            query_string += "*";
        } else {
            // is_all = false: Expression AS Variable.
            if expressions.is_empty() {
                // todo: Return Error. Invalid syntax.
            }
            // let expression = expressions[0];
        }

        // Expression AS Variable.
        for expression in expressions {
            todo!()
        }

        query_string
    }

    /// Order: `order by` sort_items
    fn visit_order(&mut self, sort_items: Vec<(Expr, Option<String>)>) -> Self::Output {
        let mut query_string = "ORDER BY ".to_string();

        // todo: notice the first expression is different from others.

        query_string
    }

    /// Match: `[OPTIONAL] MATCH` Pattern Where.
    fn visit_match(
        &mut self,
        is_optional: bool,
        pattern: Box<CypherNode>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        let mut query_string = String::new();

        // OPTIONAL MATCH
        if is_optional {
            query_string += "OPTIONAL "
        }
        query_string += "MATCH ";

        // Pattern
        query_string += &self.visit(pattern);

        // Where Clause
        if let Some(where_clause) = where_clause {
            query_string += " WHERE ";
            // todo: impl Expr to String.
        }

        query_string
    }

    /// UNWIND: `UNWIND` expression AS variable.
    fn visit_unwind(
        &mut self,
        expression: Expr,
        variable: crate::common::Variable,
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

    fn visit_delete(&mut self, is_detach: bool, expressions: Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_set(
        &mut self,
        property_set: Vec<(crate::common::PropertyExpression, Expr)>,
        variable_set: Vec<(Variable, Expr)>,
        variable_add: Vec<(Variable, Expr)>,
        label_set: Vec<(Variable, Vec<NodeLabel>)>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_explicit_procedure_invocation(
        &mut self,
        procedure_name: (crate::common::NameSpace, Variable),
        expressions: Vec<Expr>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_implicit_procedure_invocation(
        &mut self,
        procedure_name: (crate::common::NameSpace, Variable),
    ) -> Self::Output {
        todo!()
    }

    fn visit_yield_items(
        &mut self,
        yield_items: Vec<(Option<Variable>, Variable)>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        todo!()
    }

    fn visit_remove(
        &mut self,
        variable_remove: Vec<(Variable, Vec<NodeLabel>)>,
        property_remove: Vec<crate::common::PropertyExpression>,
    ) -> Self::Output {
        todo!()
    }

    /// Pattern: PatternPart+
    fn visit_pattern(&mut self, pattern_parts: Vec<Box<CypherNode>>) -> Self::Output {
        let mut query_string = String::new();

        // let first_pattern = pattern_parts.get(0).unwrap();
        // let first_string = self.visit(first_pattern);

        // query_string += &first_string;

        // Pattern: Vec<PatternPart>, pattern_parts length >= 1
        for pattern_part in pattern_parts.into_iter() {
            query_string += ",";
            query_string += &self.visit(pattern_part);
        }

        query_string
    }

    /// ### PatternPart
    ///
    /// Variable = AnonymousPatternPart
    ///
    /// AnonumousPatternPart : PatternElement.
    ///
    /// PatternPart: (Varibale =)? pattern_element
    fn visit_pattern_part(
        &mut self,
        var: Option<Variable>,
        pattern_element: Box<CypherNode>,
    ) -> Self::Output {
        let mut query_string = String::new();

        // todo: need to construct variable.
        if let Some(var) = var {
            query_string += &var.get_name();
            query_string += "=";
        }

        query_string += &self.visit(pattern_element);

        query_string
    }

    /// PatternElement
    ///
    /// NodePattern (RelationshipPattern NodePattern)*
    fn visit_pattern_element(
        &mut self,
        parentheses: i32,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    ) -> Self::Output {
        let mut query_string = String::new();

        // NodePattern
        let (node_pattern, relationships) = pattern_element;
        query_string += &self.visit(node_pattern);

        // RelationShips
        for (relationship, node) in relationships {
            // RelationShip Pattern
            query_string += " ";
            query_string += &self.visit(relationship);

            // Node Pattern
            query_string += " ";
            query_string += &self.visit(node)
        }

        query_string
    }

    /// NodePattern: (Varibale? (:label)*, Properties)
    fn visit_node_pattern(
        &mut self,
        var: Option<Variable>,
        vertex_labels: Vec<Label>,
        properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        let mut query_string = String::new();

        if let Some(var) = var {
            query_string += &var.get_name();
        }

        query_string
    }

    fn visit_relationship_pattern(
        &mut self,
        direction: crate::common::RelationshipDirection,
        var: Option<crate::common::Variable>,
        edge_labels: Vec<crate::meta::Label>,
        range: (Option<i32>, Option<i32>),
        properties: Option<(crate::common::Property, crate::common::FieldValue)>,
    ) -> Self::Output {
        todo!()
    }
}
