use super::{
    constants,
    cypher::{CypherNode, CypherNodeVisitor},
    expr_gen::ExprGenerator,
};
use crate::{
    common::{
        DataKind, Expr, ExprKind, NameSpace, RandomGenerator, RelationshipDirection,
        VariableGenerator,
    },
    meta::GraphSchema,
};

pub struct CypherGenerator {
    random: RandomGenerator,
    // limit: total expression complexity.
    pub limit: i32,
    pub graph_schema: GraphSchema,
    pub variables: VariableGenerator,
}

impl CypherGenerator {
    pub fn new_schema(graph_schema: &GraphSchema) -> Self {
        CypherGenerator {
            graph_schema: graph_schema.clone(),
            random: RandomGenerator::new(),
            variables: VariableGenerator::new(),
            limit: constants::DEFAULT_QUERY_LIMIT,
        }
    }
}

impl CypherGenerator {
    /// Generator Expr SubQuery.
    pub fn exec(&mut self) -> CypherNode {
        self.variables = VariableGenerator::new();
        self.visit_query()
    }

    /// Generator RegularQuery
    pub fn visit(&mut self) -> CypherNode {
        // init the limit parameter each new cypher.
        self.limit = constants::DEFAULT_QUERY_LIMIT;
        self.variables = VariableGenerator::new();
        self.visit_regular_query()
    }

    pub fn call_query(&mut self) -> CypherNode {
        // StandaloneCall
        self.limit = constants::DEFAULT_QUERY_LIMIT;
        self.variables = VariableGenerator::new();
        self.visit_standalone_call()
    }

    /// Pattern: RelationShipsPattern
    pub fn expr_relation_pattern(&mut self) -> CypherNode {
        self.visit_pattern_element()
    }

    /// Pattern: (Variable=)? RelationshipsPattern
    pub fn expr_pattern(&mut self) -> CypherNode {
        self.visit_pattern_part()
    }

    // #[inline]
    pub fn gen_where_expression(&mut self) -> Option<Expr> {
        if self.limit <= 0 || self.random.bool() {
            return None;
        }
        let mut expr_generator = ExprGenerator::new(self);
        Some(expr_generator.visit())
    }
}

impl CypherGenerator {
    /// Generator Property Expression.
    fn gen_property_expr(&mut self, kind: DataKind) -> Option<Expr> {
        let var = self.variables.get_target_variable(kind.clone());

        match var {
            Ok(var) => {
                let prop = if kind == DataKind::Vertex {
                    self.graph_schema.random_vertex_property(&mut self.random)
                } else {
                    self.graph_schema.random_edge_property(&mut self.random)
                };

                if let Some(prop) = prop {
                    let kind =
                        ExprKind::Property(Box::new(Expr::from(ExprKind::Variable(var))), prop);
                    Some(Expr::from(kind))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl CypherNodeVisitor for CypherGenerator {
    type Output = CypherNode;

    /// query: regular_query | standaloneCall
    fn visit_query(&mut self) -> Self::Output {
        let query = self.visit_regular_query();

        CypherNode::Query {
            query: Box::new(query),
        }
    }

    // RegularQuery: SingleQuery Union*
    fn visit_regular_query(&mut self) -> Self::Output {
        let single_query = self.visit_single_query();

        let mut union_all = vec![];
        for _ in 0..self.random.d2() {
            let single_union = self.visit_union();
            union_all.push(Box::new(single_union));
        }

        CypherNode::RegularQuery {
            single_query: Box::new(single_query),
            union_all,
        }
    }

    // StandaloneCall: CALL (ExplictProcedureInvocation | ImplicitProcedureInvocation) (YIELD *|YieldItems)?
    fn visit_standalone_call(&mut self) -> Self::Output {
        let procedure_node = if self.random.bool() {
            self.visit_explicit_procedure_invocation()
        } else {
            self.visit_implicit_procedure_invocation()
        };

        let yield_items = if self.random.bool() {
            if self.random.bool() {
                (true, None)
            } else {
                let yield_node = self.visit_yield_items();
                (true, Some(Box::new(yield_node)))
            }
        } else {
            (false, None)
        };

        CypherNode::StandaloneCall {
            procedure: Box::new(procedure_node),
            yield_items,
        }
    }

    fn visit_union(&mut self) -> Self::Output {
        let is_all = self.random.bool();

        let sub_query = self.visit_single_query();

        CypherNode::Union {
            union_all: Some((is_all, Box::new(sub_query))),
        }
    }

    // SinglePartQuery: SinglePartQuery | MultiPartQuery.
    fn visit_single_query(&mut self) -> Self::Output {
        let single_query = if self.random.bool() {
            self.visit_single_part_query()
        } else {
            self.visit_multi_part_query()
        };

        CypherNode::SingleQuery {
            part_query: Box::new(single_query),
        }
    }

    /// SinglePartQuery: ReadingClause* Return | ReadingClause* UpdatingClause+ Return?
    fn visit_single_part_query(&mut self) -> Self::Output {
        if self.random.bool() {
            let reading_number = self.random.d2();
            let mut reading_clauses = vec![];
            for _ in 0..reading_number {
                let reading_clause = self.visit_reading_clause();
                reading_clauses.push(Box::new(reading_clause));
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
                let reading_clause = self.visit_reading_clause();
                reading_clauses.push(Box::new(reading_clause));
            }
            let updating_clause = self.visit_updating_clause();
            updating_clauses.push(Box::new(updating_clause));

            for _ in 0..self.random.d2() {
                let updating_clause = self.visit_updating_clause();

                updating_clauses.push(Box::new(updating_clause));
            }

            let return_clause = if self.random.bool() {
                let return_clause = self.visit_return();

                Some(Box::new(return_clause))
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
        let mut multi_part = vec![];

        let with_number = self.random.d2();
        for _ in 0..with_number + 1 {
            let mut reading_clause = vec![];
            let mut updating_clause = vec![];
            let reading_number = self.random.d2();
            let updating_number = self.random.d2();

            for _ in 0..reading_number {
                let reading_query = self.visit_reading_clause();
                reading_clause.push(Box::new(reading_query));
            }

            for _ in 0..updating_number {
                let updating_query = self.visit_updating_clause();
                updating_clause.push(Box::new(updating_query));
            }

            let with_clause = self.visit_with();
            let with_query = Box::new(with_clause);
            multi_part.push((reading_clause, updating_clause, with_query));
        }

        let single_part = self.visit_single_part_query();

        CypherNode::MultiPartQuery {
            multi_part,
            single_part: Box::new(single_part),
        }
    }

    fn visit_with(&mut self) -> Self::Output {
        let projection_body_query = self.visit_projection_body();
        let projection_body = Box::new(projection_body_query);

        let where_clause = self.gen_where_expression();

        CypherNode::With {
            projection_body,
            where_clause,
        }
    }

    // in_query_call: call procedure.
    fn visit_in_query_call(&mut self) -> Self::Output {
        let procedure_node = self.visit_explicit_procedure_invocation();

        // YieldItems
        let yield_items = if self.random.bool() {
            let yield_items_node = self.visit_yield_items();
            Some(Box::new(yield_items_node))
        } else {
            None
        };

        CypherNode::InQueryCall {
            explicit_proceduce_invocation: Box::new(procedure_node),
            yield_items,
        }
    }

    /// ExplicitProcedureInvocation: ProcedureName ( Expression* )
    fn visit_explicit_procedure_invocation(&mut self) -> Self::Output {
        // TODO: need to implement NameSpace and SybolicName.
        let name_space = NameSpace::new();
        let symbolic_name = self.variables.get_procedure_method();

        // expressions generator.
        let mut expressions = Vec::new();
        if self.random.bool() {
            let loop_number = self.random.range(1, 3);
            let mut expr_generator = ExprGenerator::new(self);

            for _ in 0..loop_number {
                let expr = expr_generator.visit();
                expressions.push(expr);
            }
        }
        CypherNode::ExplicitProcedureInvocation {
            procedure_name: (name_space, symbolic_name),
            expressions,
        }
    }

    fn visit_implicit_procedure_invocation(&mut self) -> Self::Output {
        let name_space = NameSpace::new();
        let symbolic_name = self.variables.get_procedure_method();

        CypherNode::ImplicitProcedureInvocation {
            procedure_name: (name_space, symbolic_name),
        }
    }

    fn visit_yield_items(&mut self) -> Self::Output {
        let mut yield_items = vec![];

        // first yield_item: (ProcedureResultField AS)* variable.
        let first_variable = self.variables.new_variable();
        if self.random.bool() {
            let procedure_result = self.variables.get_procedure_result();
            yield_items.push((Some(procedure_result), first_variable));
        } else {
            yield_items.push((None, first_variable));
        }

        // yield_item*
        for _ in 0..self.random.d2() {
            let variable = self.variables.new_variable();
            if self.random.bool() {
                let procedure_result = self.variables.get_procedure_result();
                yield_items.push((Some(procedure_result), variable));
            } else {
                yield_items.push((None, variable));
            }
        }

        // where_clause
        let where_clause = if self.random.bool() {
            let mut expr_generator = ExprGenerator::new(self);
            let where_expr = expr_generator.visit();
            Some(where_expr)
        } else {
            None
        };

        CypherNode::YieldItems {
            yield_items,
            where_clause,
        }
    }

    fn visit_reading_clause(&mut self) -> Self::Output {
        let reading_clause = match self.random.d6() {
            0 => self.visit_match(),
            1 => self.visit_unwind(),
            // default: match clause.
            _ => self.visit_match(),
        };

        CypherNode::ReadingClause {
            reading_clause: Box::new(reading_clause),
        }
    }

    /// Match Clause: Optional MATCH **pattern** [WHERE clause]
    fn visit_match(&mut self) -> Self::Output {
        let is_optional = self.random.bool();

        let pattern_node = self.visit_pattern();
        let pattern = Box::new(pattern_node);

        // generator where expression.
        let where_clause = self.gen_where_expression();

        CypherNode::Match {
            is_optional,
            pattern,
            where_clause,
        }
    }

    // unwind: UNWIND expression AS variable.
    fn visit_unwind(&mut self) -> Self::Output {
        let mut expr_generator = ExprGenerator::new(self);
        let expression = expr_generator.visit();
        let var_kind = expression.kind.get_kind();
        let variable = self.variables.new_kind_variable(var_kind);

        CypherNode::Unwind {
            expression,
            variable,
        }
    }

    fn visit_updating_clause(&mut self) -> Self::Output {
        let updating_clause = match self.random.d6() {
            0 => self.visit_create(),
            1 => self.visit_merge(),
            2 => self.visit_delete(),
            3 => self.visit_set(),
            4 => self.visit_remove(),
            // default: create clause.
            _ => self.visit_create(),
        };

        CypherNode::UpdatingClause {
            updating_clause: Box::new(updating_clause),
        }
    }

    fn visit_create(&mut self) -> Self::Output {
        let pattern = self.visit_pattern();

        CypherNode::Create {
            pattern: Box::new(pattern),
        }
    }

    // merge: MERGE pattern_part (merge_action)*; merge_action: on match|create set.
    fn visit_merge(&mut self) -> Self::Output {
        let pattern_part_node = self.visit_pattern_part();
        let pattern_part = Box::new(pattern_part_node);

        let mut merge_actions = Vec::new();

        for _ in 0..self.random.d2() {
            let merge_action = self.visit_set();

            let opt = if self.random.bool() {
                "MATCH ".to_string()
            } else {
                "CREATE ".to_string()
            };
            merge_actions.push((opt, Box::new(merge_action)));
        }

        CypherNode::Merge {
            pattern_part,
            merge_actions,
        }
    }

    /// ### delete
    ///
    /// detach? delete Vec\<expressions>
    fn visit_delete(&mut self) -> Self::Output {
        let is_detach = self.random.bool();

        let mut expressions = Vec::new();

        for _ in 0..self.random.range(1, 3) {
            let mut expr_generator = ExprGenerator::new(self);
            let expr = expr_generator.visit();
            expressions.push(expr);
        }

        CypherNode::Delete {
            is_detach,
            expressions,
        }
    }

    /// ### Set
    ///
    /// set (PropertyExpression = Expression | Variable = Expression | Variable += Expression | Variable = NodeLabels)*
    ///
    /// PropertyExpression: Atom (PropertyLookUp)*
    fn visit_set(&mut self) -> Self::Output {
        let mut property_set = vec![];
        let mut variable_set = vec![];
        let mut variable_add = vec![];
        let mut label_set = vec![];

        // first set_item
        for _ in 0..self.random.d2() + 1 {
            match self.random.d6() % 3 {
                0 => {
                    // Correct PropertyExpression.
                    let property = if self.random.d9() > 1 {
                        // Vertex Property
                        self.gen_property_expr(DataKind::Vertex)
                    } else {
                        // Edge Property
                        self.gen_property_expr(DataKind::Edge)
                    };

                    if property.is_none() {
                        continue;
                    }

                    let mut expr_generator = ExprGenerator::new(self);
                    let expression = expr_generator.visit();
                    property_set.push((property.unwrap(), expression));
                }
                1 => {
                    let variable = self.variables.get_old_variable();
                    let mut expr_generator = ExprGenerator::new(self);
                    let expression = expr_generator.visit();
                    if self.random.bool() {
                        variable_set.push((variable, expression));
                    } else {
                        variable_add.push((variable, expression));
                    }
                }
                2 => {
                    let variable = self.variables.get_target_variable(DataKind::Vertex);
                    let var = match variable {
                        Ok(var) => var,
                        _ => self.variables.get_old_variable(),
                    };
                    // NodeLabels: NodeLabel+
                    let mut node_labels = vec![];

                    let node_label = self.graph_schema.rand_vertex_label(&mut self.random);
                    node_labels.push(node_label);

                    label_set.push((var, node_labels));
                }
                _ => {}
            }
        }

        let set_size =
            property_set.len() + variable_set.len() + variable_add.len() + label_set.len();
        if set_size == 0 {
            return self.visit_set();
        }

        CypherNode::Set {
            property_set,
            variable_set,
            variable_add,
            label_set,
        }
    }

    // remove: remove (variable Nodelabel* | PropertyExpression)+
    fn visit_remove(&mut self) -> Self::Output {
        let mut variable_remove = vec![];
        let mut property_remove = vec![];

        for _ in 0..self.random.d2() + 1 {
            if self.random.bool() {
                let variable = self.variables.get_target_variable(DataKind::Vertex);
                let var = match variable {
                    Ok(var) => var,
                    _ => self.variables.get_old_variable(),
                };

                let mut node_labels = vec![];
                let node_label = self.graph_schema.rand_vertex_label(&mut self.random);
                node_labels.push(node_label);

                variable_remove.push((var, node_labels));
            } else {
                let property = if self.random.d9() > 1 {
                    // Vertex Property
                    self.gen_property_expr(DataKind::Vertex)
                } else {
                    // Edge Property
                    self.gen_property_expr(DataKind::Edge)
                };

                if property.is_none() {
                    continue;
                }

                property_remove.push(property.unwrap());
            }
        }

        let set_size = variable_remove.len() + property_remove.len();
        if set_size == 0 {
            return self.visit_remove();
        }

        CypherNode::Remove {
            variable_remove,
            property_remove,
        }
    }

    /// Return clause: return projection_body.
    fn visit_return(&mut self) -> Self::Output {
        let projection_body = self.visit_projection_body();

        CypherNode::Return {
            projection_body: Box::new(projection_body),
        }
    }

    fn visit_projection_body(&mut self) -> Self::Output {
        // DISTINCT
        let is_distinct = self.random.d6() == 1;

        // ProjectionItems
        let projection_items_node = self.visit_projection_items();
        let projection_items = Box::new(projection_items_node);

        // order:
        let order = if self.random.low_prob_bool() {
            let order_node = self.visit_order();
            Some(Box::new(order_node))
        } else {
            None
        };
        let skip = if self.random.low_prob_bool() {
            let mut expr_generator = ExprGenerator::new(self);
            let skip_expression = expr_generator.visit();
            Some(skip_expression)
        } else {
            None
        };
        let limit = if self.random.low_prob_bool() {
            let mut expr_generator = ExprGenerator::new(self);
            let limit_expression = expr_generator.visit();
            Some(limit_expression)
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

    /// ProjectionItems: *(,ProjectionItem)*|ProjectionItem+
    fn visit_projection_items(&mut self) -> Self::Output {
        let mut expressions = Vec::new();
        let is_all = if self.random.d9() == 1 {
            true
        } else {
            let mut expr_generator = ExprGenerator::new(self);
            let expression = expr_generator.visit();

            let var = if self.random.bool() {
                let expr_kind = expression.kind.get_kind();
                let variable = self.variables.new_kind_variable(expr_kind);

                Some(variable)
            } else {
                None
            };
            expressions.push((expression, var));
            false
        };

        // projection_items
        for _ in 0..self.random.d2() {
            let mut expr_generator = ExprGenerator::new(self);
            let expression = expr_generator.visit();

            let var = if self.random.bool() {
                let expr_kind = expression.kind.get_kind();
                let variable = self.variables.new_kind_variable(expr_kind);

                Some(variable)
            } else {
                None
            };
            expressions.push((expression, var));
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

        let mut expr_generator = ExprGenerator::new(self);
        let first_expression = expr_generator.visit();

        let rule = if self.random.bool() {
            let rule_string = sort_rules[self.random.d2() as usize].to_string();
            Some(rule_string)
        } else {
            None
        };
        sort_items.push((first_expression, rule));

        for _ in 0..self.random.d2() {
            let mut expr_generator = ExprGenerator::new(self);
            let expression = expr_generator.visit();

            let rule = if self.random.bool() {
                let rule_string = sort_rules[self.random.d2() as usize].to_string();
                Some(rule_string)
            } else {
                None
            };
            sort_items.push((expression, rule))
        }

        CypherNode::Order { sort_items }
    }

    // Pattern: PatternPart+
    fn visit_pattern(&mut self) -> Self::Output {
        let mut pattern_parts = vec![];

        let pattern_part_node = self.visit_pattern_part();
        pattern_parts.push(Box::new(pattern_part_node));

        for _ in 0..self.random.d2() {
            let pattern_part_node = self.visit_pattern_part();
            pattern_parts.push(Box::new(pattern_part_node));
        }

        CypherNode::Pattern { pattern_parts }
    }

    // PatternPart: (Variable =)? pattern_element
    fn visit_pattern_part(&mut self) -> Self::Output {
        let var = if self.random.bool() {
            let variable = self.variables.new_kind_variable(DataKind::Path);
            Some(variable)
        } else {
            None
        };

        let pattern_element_node = self.visit_pattern_element();
        CypherNode::PatternPart {
            var,
            pattern_element: Box::new(pattern_element_node),
        }
    }

    // pattern_element: NodePattern (RelationshipPattern NodePattern)*
    fn visit_pattern_element(&mut self) -> Self::Output {
        let parenthesis = self.random.d12() < 1;

        let node_pattern_node = self.visit_node_pattern();
        let node_pattern = Box::new(node_pattern_node);

        let mut pattern_element_chain = vec![];
        for _ in 0..self.random.d2() {
            let relationship_node = self.visit_relationship_pattern();
            let node = self.visit_node_pattern();

            pattern_element_chain.push((Box::new(relationship_node), Box::new(node)));
        }

        // for _ in 0..parentheses_number {
        //     pattern_element_string += ")";
        // }

        // let x = (0..parentheses_number).into_iter().map(|_| ")").collect::<String>();
        CypherNode::PatternElement {
            parenthesis,
            pattern_element: (node_pattern, pattern_element_chain),
        }
    }

    // NodePattern: ( Variable? (:label)* Properties)
    fn visit_node_pattern(&mut self) -> Self::Output {
        let var = if self.random.bool() {
            let variable = self.variables.new_kind_variable(DataKind::Vertex);
            Some(variable)
        } else {
            None
        };

        // use exists node label.
        let mut vertex_labels = vec![];

        // node label.
        let node_property = if self.random.bool() {
            let node_label = self.graph_schema.rand_vertex_label(&mut self.random);
            let node_property = node_label.random_property(&mut self.random);
            vertex_labels.push(node_label);
            node_property
        } else {
            None
        };

        // previous label's properties.
        let properties = if self.random.bool() {
            if let Some(prop) = node_property {
                let property_value = prop.default_value();
                Some((prop, property_value))
            } else {
                None
            }
        } else {
            None
        };

        CypherNode::NodePattern {
            var,
            vertex_labels,
            properties,
        }
    }

    /// RelationShips Pattern.
    fn visit_relationship_pattern(&mut self) -> Self::Output {
        let direction = match self.random.d6() {
            0 => RelationshipDirection::Left,
            1 => RelationshipDirection::Right,
            2 => RelationshipDirection::Both,
            3 => RelationshipDirection::None,
            _ => RelationshipDirection::None,
        };

        let var = if self.random.bool() {
            let variable = self.variables.new_kind_variable(DataKind::Edge);
            Some(variable)
        } else {
            None
        };
        let mut edge_labels = vec![];
        if self.random.bool() {
            let relation_label = self.graph_schema.rand_edge_label(&mut self.random);
            edge_labels.push(relation_label);
        }

        // multi edge labels.
        // if self.random.bool() {
        //     for _ in 0..self.random.d2() {
        //         let relation_label = self.graph_schema.rand_edge_label(&mut self.random);
        //         edge_labels.push(relation_label);
        //     }
        // }

        let (is_range, range) = if self.random.bool() {
            //
            let range_start = if self.random.bool() {
                Some(self.random.d2())
            } else {
                None
            };

            let range_end = if self.random.bool() {
                let is_range_end = true;
                let range_end = if self.random.bool() {
                    if let Some(range_start) = range_start {
                        Some(range_start + self.random.d2())
                    } else {
                        Some(self.random.d2())
                    }
                } else {
                    None
                };

                Some((is_range_end, range_end))
            } else {
                None
            };

            (true, (range_start, range_end))
        } else {
            (false, (None, None))
        };

        let properties = if self.random.bool() && !edge_labels.is_empty() {
            let edge_property = edge_labels[0].clone().random_property(&mut self.random);
            if let Some(prop) = edge_property {
                let property_value = prop.default_value();
                Some((prop, property_value))
            } else {
                None
            }
        } else {
            None
        };

        CypherNode::RelationshipPattern {
            direction,
            var,
            edge_labels,
            is_range,
            range,
            properties,
        }
    }
}
