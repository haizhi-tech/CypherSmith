use super::cypher::{ConvertVisitor, CypherNode};

use crate::{
    common::{Expr, FieldValue, NameSpace, Property, RelationshipDirection, Variable},
    meta::Label,
};

pub struct TransformVisitor {
    // cypher_string: String,
}

impl TransformVisitor {
    pub fn new() -> TransformVisitor {
        Self {
            // cypher_string: "".to_string(),
        }
    }

    pub fn exec(&mut self, query: Box<CypherNode>) -> String {
        self.visit(query)
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

    /// StandaloneCall: `CALL` (ExplicitProcedureInvocation|ImplicitProcedureInvocation),  (YIELD ('*' | YieldItems))?
    fn visit_standalone_call(
        &mut self,
        procedure: Box<CypherNode>,
        yield_items: (bool, Option<Box<CypherNode>>),
    ) -> Self::Output {
        let mut query_string = "CALL ".to_string();
        query_string += &self.visit(procedure);

        match yield_items.0 {
            true => {
                if let Some(yield_items) = yield_items.1 {
                    query_string += &self.visit(yield_items);
                } else {
                    query_string += "*";
                }
            }
            false => {}
        }

        query_string
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

    /// MultiPartQuery: (ReadingClause* UpdatingClause* With)+ SinglePartQuery
    fn visit_multi_part_query(
        &mut self,
        multi_part: Vec<(Vec<Box<CypherNode>>, Vec<Box<CypherNode>>, Box<CypherNode>)>,
        single_part: Box<CypherNode>,
    ) -> Self::Output {
        let mut query_string = String::new();

        for (reading_clauses, updating_clauses, with_clause) in multi_part.into_iter() {
            // ReadingClause*
            for reading_clause in reading_clauses.into_iter() {
                query_string += &self.visit(reading_clause);
                query_string += " ";
            }
            // UpdatingClause*
            for updating_clause in updating_clauses.into_iter() {
                query_string += &self.visit(updating_clause);
                query_string += " ";
            }
            // With
            query_string += &self.visit(with_clause);
            query_string += " ";
        }

        query_string += &self.visit(single_part);

        query_string
    }

    /// With: `WITH` ProjectionBody Where?
    fn visit_with(
        &mut self,
        projection_body: Box<CypherNode>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        // WITH ProjectionBody
        let mut with_string = "WITH".to_string();
        with_string += &self.visit(projection_body);

        // Where Expr
        if let Some(where_clause) = where_clause {
            with_string += " WHERE ";
            with_string += &where_clause.to_string();
        }

        with_string
    }

    /// Union: `Union` `ALL`? SinglePartQuery.
    fn visit_union(&mut self, union_all: Option<(bool, Box<CypherNode>)>) -> Self::Output {
        let mut union_string = String::new();

        if let Some((union_all, single_part)) = union_all {
            if union_all {
                union_string += "UNION ALL ";
            } else {
                union_string += "UNION ";
            }
            union_string += &self.visit(single_part);
        }

        union_string
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

        if let Some(order_node) = order {
            query_string += " ";
            query_string += &self.visit(order_node);
        }
        if let Some(skip_expr) = skip {
            query_string += " ";
            query_string += &skip_expr.to_string();
        }
        if let Some(limit_expr) = limit {
            query_string += " ";
            query_string += &limit_expr.to_string();
        }

        query_string
    }

    /// ### ProjectionItems:
    ///
    /// *(,ProjectionItem)*|ProjectionItem+
    fn visit_projection_items(
        &mut self,
        is_all: bool,
        expressions: Vec<(Expr, Option<Variable>)>,
    ) -> Self::Output {
        let mut query_string = String::new();

        // Expression AS Variable.
        let expr_string = expressions
            .into_iter()
            .map(|(expr, var)| {
                let mut x = expr.to_string();
                if let Some(var) = var {
                    x += " AS ";
                    x += &var.get_name();
                }
                x
            })
            .collect::<Vec<_>>()
            .join(",");

        if is_all {
            // is_all = true: *
            query_string += "*";
            if !expr_string.is_empty() {
                query_string += ",";
            }
        }

        query_string += &expr_string;

        query_string
    }

    /// Order: `order by` sort_items
    fn visit_order(&mut self, sort_items: Vec<(Expr, Option<String>)>) -> Self::Output {
        let mut query_string = "ORDER BY ".to_string();

        if sort_items.is_empty() {
            unreachable!()
        }

        for (expr, rule) in sort_items.into_iter() {
            query_string += &expr.to_string();
            if let Some(rule) = rule {
                query_string += " ";
                query_string += &rule;
            }
        }

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
            query_string += &where_clause.to_string();
        }

        query_string
    }

    /// UNWIND: `UNWIND` expression AS variable.
    fn visit_unwind(&mut self, expression: Expr, variable: Variable) -> Self::Output {
        let mut unwind_string = "UNWIND ".to_string();
        unwind_string += &expression.to_string();
        unwind_string += " AS ";
        unwind_string += &variable.get_name();
        unwind_string
    }

    /// InQueryCall: `CALL` procedure.
    fn visit_in_query_call(
        &mut self,
        explicit_proceduce_invocation: Box<CypherNode>,
        yield_items: Option<Box<CypherNode>>,
    ) -> Self::Output {
        let mut query_string = "CALL ".to_string();
        query_string += &self.visit(explicit_proceduce_invocation);

        // YieldItems
        if let Some(yield_items) = yield_items {
            query_string += " YIELD ";
            query_string += &self.visit(yield_items);
        }

        query_string
    }

    /// Create: Create Pattern
    fn visit_create(&mut self, pattern: Box<CypherNode>) -> Self::Output {
        let mut create_string = "CREATE ".to_string();
        create_string += &self.visit(pattern);
        create_string
    }

    /// Merge Merge PatternPart (MergeAction)*
    ///
    /// MergeAction: ON (Match|Create) Set
    fn visit_merge(
        &mut self,
        pattern_part: Box<CypherNode>,
        merge_actions: Vec<(String, Box<CypherNode>)>,
    ) -> Self::Output {
        let mut merge_string = "MERGE ".to_string();
        // PatternPart
        merge_string += &self.visit(pattern_part);

        // MergeAction*
        for (opt, merge_action) in merge_actions {
            merge_string += " ON ";
            merge_string += &opt;
            merge_string += &self.visit(merge_action);
        }

        merge_string
    }

    /// ### Delete
    ///
    /// `Detach`? Delete (Expression)*
    fn visit_delete(&mut self, is_detach: bool, expressions: Vec<Expr>) -> Self::Output {
        let mut delete_string = if is_detach {
            "DETACH DELETE ".to_string()
        } else {
            "DELETE ".to_string()
        };

        if expressions.is_empty() {
            unreachable!()
        }

        let exprs = expressions
            .into_iter()
            .map(|expr| expr.to_string())
            .collect::<Vec<_>>()
            .join(",");

        delete_string += &exprs;

        delete_string
    }

    /// ### Set:
    ///
    /// Set SetItem+
    ///
    /// SetItem:  (Property = Expression | Variable = Expression | Variable += Expression | Variable = NodeLabels)
    fn visit_set(
        &mut self,
        property_set: Vec<(Expr, Expr)>,
        variable_set: Vec<(Variable, Expr)>,
        variable_add: Vec<(Variable, Expr)>,
        label_set: Vec<(Variable, Vec<Label>)>,
    ) -> Self::Output {
        let mut set_string = "SET ".to_string();

        let property_string = property_set.into_iter().map(|(property, expr)| {
            let mut ret = property.to_string();
            ret += "=";
            ret += &expr.to_string();
            ret
        });

        // Variable = Expression
        let variable_string = variable_set.into_iter().map(|(var, expr)| {
            let mut ret = var.get_name();
            ret += "=";
            ret += &expr.to_string();
            ret
        });

        // Variable += Expression
        let variable_add_string = variable_add.into_iter().map(|(var, expr)| {
            let mut ret = var.get_name();
            ret += "+=";
            ret += &expr.to_string();
            ret
        });

        // Variable = NodeLabels
        let label_string = label_set.into_iter().map(|(var, labels)| {
            let mut ret = var.get_name();
            for label in labels {
                ret += ":";
                ret += &label.get_name();
            }
            ret
        });

        // collect string
        let set_items_string = property_string
            .chain(variable_add_string)
            .chain(variable_string)
            .chain(label_string)
            .collect::<Vec<String>>()
            .join(",");

        set_string += &set_items_string;

        set_string
    }

    /// ### ExplicitProcedureInvocation
    ///
    /// SymbolicName.ProcedureName ( Expression* )
    fn visit_explicit_procedure_invocation(
        &mut self,
        procedure_name: (NameSpace, Variable),
        expressions: Vec<Expr>,
    ) -> Self::Output {
        // NameSpace.ScymbolicName: eg: atlas.shortestpath
        let mut query_string = procedure_name.0.get_name();
        query_string += ".";
        query_string += &procedure_name.1.get_name();

        query_string += "(";
        if !expressions.is_empty() {
            let exprs = expressions
                .into_iter()
                .map(|expr| expr.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_string += &exprs;
        }
        query_string += ")";

        query_string
    }

    /// ### ImplicitProcedureInvocation
    ///
    /// SymbolicName.ProcedureName
    fn visit_implicit_procedure_invocation(
        &mut self,
        procedure_name: (NameSpace, Variable),
    ) -> Self::Output {
        let mut query_string = procedure_name.0.get_name();
        query_string += ".";
        query_string += &procedure_name.1.get_name();

        query_string
    }

    /// ### YieldItems
    ///
    /// YieldItem (,YieldItem)* Where?
    ///
    /// YieldItem: (ProcedureResultField AS)? Variable
    fn visit_yield_items(
        &mut self,
        yield_items: Vec<(Option<Variable>, Variable)>,
        where_clause: Option<Expr>,
    ) -> Self::Output {
        let mut query_string = String::new();

        let yield_string = yield_items
            .into_iter()
            .map(|(res, var)| {
                let mut x = String::new();
                if let Some(res) = res {
                    x += &res.get_name();
                    x += " AS ";
                }
                x += &var.get_name();
                x
            })
            .collect::<Vec<_>>()
            .join(",");

        query_string += &yield_string;

        // Where Clasue
        if let Some(where_clause) = where_clause {
            query_string += " WHERE ";
            query_string += &where_clause.to_string();
        }

        query_string
    }

    /// Remove
    ///
    /// `REMOVE` RemoveItems+
    ///
    /// RemoveItem: (Variable NodeLabels| PropertyExpression)
    fn visit_remove(
        &mut self,
        variable_remove: Vec<(Variable, Vec<Label>)>,
        property_remove: Vec<Expr>,
    ) -> Self::Output {
        let mut remove_string = "REMOVE ".to_string();

        // remove items cannot be null.
        if variable_remove.is_empty() && property_remove.is_empty() {
            unreachable!()
        }

        let variable = variable_remove.into_iter().map(|(var, labels)| {
            let mut x = var.get_name();
            for label in labels {
                x += ":";
                x += &label.get_name();
            }
            x
        });

        let property = property_remove
            .into_iter()
            .map(|property| property.to_string());

        let res_chain = variable.chain(property).collect::<Vec<_>>().join(",");

        remove_string += &res_chain;

        remove_string
    }

    /// Pattern: PatternPart+
    fn visit_pattern(&mut self, pattern_parts: Vec<Box<CypherNode>>) -> Self::Output {
        let mut query_string = String::new();

        if pattern_parts.is_empty() {
            unreachable!()
        }

        let pattern_string = pattern_parts
            .into_iter()
            .map(|pattern_node| self.visit(pattern_node))
            .collect::<Vec<_>>()
            .join(",");

        query_string += &pattern_string;
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
        parenthesis: bool,
        pattern_element: (Box<CypherNode>, Vec<(Box<CypherNode>, Box<CypherNode>)>),
    ) -> Self::Output {
        let mut query_string = String::new();

        // right parenthesis
        if parenthesis {
            query_string += "(";
        }

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

        // right parenthesis
        if parenthesis {
            query_string += ")";
        }

        query_string
    }

    /// ### NodePattern
    ///
    /// (Varibale? (:label)*, Properties)
    fn visit_node_pattern(
        &mut self,
        var: Option<Variable>,
        vertex_labels: Vec<Label>,
        properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        let mut query_string = "(".to_string();

        if let Some(var) = var {
            query_string += &var.get_name();
        }

        for node_label in vertex_labels {
            query_string += ":";
            query_string += &node_label.get_name();
        }

        if let Some((node_property, property_value)) = properties {
            query_string += "{";
            query_string += &node_property.get_name();
            query_string += ":";
            query_string += &property_value.to_string();
            query_string += "}";
        }
        query_string += ")";

        query_string
    }

    /// ### RelationShipPattern
    fn visit_relationship_pattern(
        &mut self,
        direction: RelationshipDirection,
        var: Option<Variable>,
        edge_labels: Vec<Label>,
        is_range: bool,
        range: (Option<i32>, Option<(bool, Option<i32>)>),
        properties: Option<(Property, FieldValue)>,
    ) -> Self::Output {
        let mut query_string = direction.left_string();

        if let Some(var) = var {
            query_string += &var.get_name();
        }

        let labels_string = edge_labels
            .into_iter()
            .map(|label| label.get_name())
            .collect::<Vec<_>>()
            .join("|:");
        query_string += ":";
        query_string += &labels_string;

        // *RangeStart..RangeEnd
        if is_range {
            query_string += "*";
            let (range_start, range_end) = range;
            if let Some(range_start) = range_start {
                query_string += &range_start.to_string();
            }
            if let Some((is_range_end, range_end)) = range_end {
                if is_range_end {
                    query_string += "..";
                    if let Some(range_end) = range_end {
                        query_string += &range_end.to_string();
                    }
                }
            }
        }

        // Property
        if let Some((edge_property, property_value)) = properties {
            query_string += "{";
            query_string += &edge_property.get_name();
            query_string += ":";
            query_string += &property_value.to_string();
            query_string += "}";
        }

        query_string += &direction.right_string();
        query_string
    }
}
