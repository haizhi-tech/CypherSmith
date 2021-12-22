use super::{
    cypher::{CypherNode, CypherNodeVisitor},
    ExprGenerator,
};
use crate::common::{
    Expression, Literal, NameSpace, NodeLabel, PropertyExpression, RandomGenerator,
    RelationshipDirection, SchemaName, VariableGenerator,
};
use crate::meta::GraphSchema;

pub struct CypherGenerator {
    // query_string: String,
    graph_schema: GraphSchema,
    random: RandomGenerator,
    variables: VariableGenerator,
}

impl CypherGenerator {
    pub fn new() -> Self {
        CypherGenerator {
            // query_string: String::new(),
            graph_schema: GraphSchema::default(),
            random: RandomGenerator::new(),
            variables: VariableGenerator::new(),
        }
    }

    pub fn new_schema(graph_schema: &GraphSchema) -> Self {
        CypherGenerator {
            graph_schema: graph_schema.clone(),
            random: RandomGenerator::new(),
            variables: VariableGenerator::new(),
        }
    }
}

impl CypherGenerator {
    pub fn visit(&mut self) -> (CypherNode, String) {
        // let (cypher_node, query_string) = self.visit_query();
        // self.query_string = query_string;
        // cypher_node
        self.visit_query()
    }

    pub fn visit_expression(&mut self) -> String {
        self.visit_property_or_labels_expression().1
    }

    // pub fn get_current_query_string(&mut self) -> String {
    //     self.query_string.clone()
    // }
    pub fn test_match_clause(&mut self) -> (CypherNode, String) {
        self.visit_match()
    }
}

impl CypherNodeVisitor for CypherGenerator {
    type Output = (CypherNode, String);

    /// query: regular_query | standaloneCall
    fn visit_query(&mut self) -> Self::Output {
        let (query, query_string) = if self.random.bool() {
            self.visit_regular_query()
        } else {
            self.visit_standalone_call()
        };

        (
            CypherNode::Query {
                query: Box::new(query),
            },
            query_string,
        )
    }

    fn visit_regular_query(&mut self) -> Self::Output {
        let mut query_string = String::new();

        let (single_query, single_query_string) = self.visit_single_query();

        query_string += &single_query_string;

        let mut union_all = vec![];
        for _ in 0..self.random.d2() {
            let (single_union, single_union_string) = self.visit_union();
            union_all.push(Box::new(single_union));
            query_string += " ";
            query_string += &single_union_string;
        }

        (
            CypherNode::RegularQuery {
                single_query: Box::new(single_query),
                union_all,
            },
            query_string,
        )
    }

    fn visit_standalone_call(&mut self) -> Self::Output {
        let mut standalone_call_string = "CALL ".to_string();

        let (procedure_node, procedure_string) = if self.random.bool() {
            self.visit_explicit_procedure_invocation()
        } else {
            self.visit_implicit_procedure_invocation()
        };
        standalone_call_string += &procedure_string;

        let yield_items = if self.random.bool() {
            standalone_call_string += " YIELD ";
            if self.random.bool() {
                standalone_call_string += "*";
                (true, None)
            } else {
                let (yield_node, yield_string) = self.visit_yield_items();
                standalone_call_string += &yield_string;
                (true, Some(Box::new(yield_node)))
            }
        } else {
            (false, None)
        };

        (
            CypherNode::StandaloneCall {
                procedure: Box::new(procedure_node),
                yield_items,
            },
            standalone_call_string,
        )
    }

    fn visit_union(&mut self) -> Self::Output {
        let mut union_string = String::new();
        let is_all = if self.random.bool() {
            union_string += "UNION ALL ";
            true
        } else {
            union_string += "UNION ";
            false
        };

        let (sub_query, sub_query_string) = self.visit_single_query();
        union_string += &sub_query_string;

        (
            CypherNode::Union {
                union_all: Some((is_all, Box::new(sub_query))),
            },
            union_string,
        )
    }

    fn visit_single_query(&mut self) -> Self::Output {
        let (single_query, single_query_string) = if self.random.bool() {
            self.visit_single_part_query()
        } else {
            self.visit_multi_part_query()
        };

        (
            CypherNode::SingleQuery {
                part_query: Box::new(single_query),
            },
            single_query_string,
        )
    }

    /// SinglePartQuery: ReadingClause* Return | ReadingClause* UpdatingClause+ Return?
    fn visit_single_part_query(&mut self) -> Self::Output {
        let mut single_part_string = String::new();

        if self.random.bool() {
            let reading_number = self.random.d2();
            let mut reading_clauses = vec![];
            for _ in 0..reading_number {
                let (reading_clause, reading_string) = self.visit_reading_clause();
                reading_clauses.push(Box::new(reading_clause));
                single_part_string += &reading_string;
                single_part_string += " ";
            }

            let (return_clause, return_string) = self.visit_return();
            single_part_string += &return_string;

            (
                CypherNode::SinglePartQuery {
                    reading_clauses,
                    updating_clauses: vec![],
                    return_clause: Some(Box::new(return_clause)),
                },
                single_part_string,
            )
        } else {
            let mut reading_clauses = vec![];
            let mut updating_clauses = vec![];
            for _ in 0..self.random.d2() {
                let (reading_clause, reading_string) = self.visit_reading_clause();
                reading_clauses.push(Box::new(reading_clause));
                single_part_string += &reading_string;
                single_part_string += " ";
            }
            let (updating_clause, updating_string) = self.visit_updating_clause();
            updating_clauses.push(Box::new(updating_clause));
            single_part_string += &updating_string;

            for _ in 0..self.random.d2() {
                let (updating_clause, updating_string) = self.visit_updating_clause();
                single_part_string += " ";
                single_part_string += &updating_string;
                updating_clauses.push(Box::new(updating_clause));
            }

            let return_clause = if self.random.bool() {
                let (return_clause, return_string) = self.visit_return();
                single_part_string += " ";
                single_part_string += &return_string;
                Some(Box::new(return_clause))
            } else {
                None
            };

            (
                CypherNode::SinglePartQuery {
                    reading_clauses,
                    updating_clauses,
                    return_clause,
                },
                single_part_string,
            )
        }
    }

    // multi_part: ((ReadingClause)* (Updating_clause)* With)+ SinglePartQuery
    fn visit_multi_part_query(&mut self) -> Self::Output {
        let mut multi_part_string = String::new();

        let mut multi_part = vec![];

        let with_number = self.random.d2();
        for _ in 0..with_number {
            let mut reading_clause = vec![];
            let mut updating_clause = vec![];
            let reading_number = self.random.d2();
            let updating_number = self.random.d2();

            for _ in 0..reading_number {
                let (reading_query, reading_string) = self.visit_reading_clause();
                reading_clause.push(Box::new(reading_query));
                multi_part_string += &reading_string;
                multi_part_string += " ";
            }

            for _ in 0..updating_number {
                let (updating_query, updating_string) = self.visit_updating_clause();
                updating_clause.push(Box::new(updating_query));
                multi_part_string += &updating_string;
                multi_part_string += " ";
            }

            let (with_clause, with_string) = self.visit_with();
            let with_query = Box::new(with_clause);
            multi_part_string += &with_string;
            multi_part_string += " ";
            multi_part.push((reading_clause, updating_clause, with_query));
        }

        let (single_part, single_part_string) = self.visit_single_part_query();
        multi_part_string += &single_part_string;

        (
            CypherNode::MultiPartQuery {
                multi_part,
                single_part: Box::new(single_part),
            },
            multi_part_string,
        )
    }

    fn visit_with(&mut self) -> Self::Output {
        let mut with_string = String::new();

        let (projection_body_query, projection_body_string) = self.visit_projection_body();
        let projection_body = Box::new(projection_body_query);
        with_string += &projection_body_string;

        let where_clause = if self.random.bool() {
            // let where_expression = Expression::new();
            // with_string += &where_expression.get_name();
            let mut expr_generator = ExprGenerator::new(self);
            let (where_string, where_expression) = expr_generator.visit();
            with_string += " WHERE ";
            with_string += &where_string;
            Some(where_expression)
        } else {
            None
        };

        (
            CypherNode::With {
                projection_body,
                where_clause,
            },
            with_string,
        )
    }

    // in_query_call: call procedure.
    fn visit_in_query_call(&mut self) -> Self::Output {
        let mut in_query_call_string = "CALL ".to_string();
        let (procedure_node, procedure_string) = self.visit_explicit_procedure_invocation();
        in_query_call_string += &procedure_string;

        // YieldItems
        let yield_items = if self.random.bool() {
            let (yield_items_node, yield_items_string) = self.visit_yield_items();
            in_query_call_string += " YIELD ";
            in_query_call_string += &yield_items_string;
            Some(Box::new(yield_items_node))
        } else {
            None
        };

        (
            CypherNode::InQueryCall {
                explicit_proceduce_invocation: Box::new(procedure_node),
                yield_items,
            },
            in_query_call_string,
        )
    }

    fn visit_explicit_procedure_invocation(&mut self) -> Self::Output {
        let mut procedure_string = String::new();
        let name_space = NameSpace::new();
        // let mut expr_generator = ExprGenerator::new(self);
        // let (where_string, where_expression) = expr_generator.visit();
        let symbolic_name = self.variables.get_procedure_method();
        procedure_string += &name_space.get_name();
        procedure_string += ".";
        procedure_string += &symbolic_name.get_name();
        procedure_string += "(";

        // expressions generator.
        let mut expressions = Vec::new();
        if self.random.bool() {
            let expression = Expression::new();
            procedure_string += &expression.get_name();
            expressions.push(expression);

            for _ in 0..self.random.d2() {
                let expression = Expression::new();
                procedure_string += ",";
                procedure_string += &expression.get_name();
                expressions.push(expression);
            }
        }
        procedure_string += ")";

        (
            CypherNode::ExplicitProcedureInvocation {
                procedure_name: (name_space, symbolic_name),
                expressions,
            },
            procedure_string,
        )
    }

    fn visit_implicit_procedure_invocation(&mut self) -> Self::Output {
        let mut procedure_string = String::new();
        let name_space = NameSpace::new();
        let symbolic_name = self.variables.get_procedure_method();
        procedure_string += &name_space.get_name();
        procedure_string += ".";
        procedure_string += &symbolic_name.get_name();
        (
            CypherNode::ImplicitProcedureInvocation {
                procedure_name: (name_space, symbolic_name),
            },
            procedure_string,
        )
    }

    fn visit_yield_items(&mut self) -> Self::Output {
        let mut yield_items_string = String::new();

        let mut yield_items = vec![];

        // first yield_item: (ProcedureResultField AS)* variable.
        let first_variable = self.variables.new_variable();
        if self.random.bool() {
            let procedure_result = self.variables.get_procedure_result();
            yield_items_string += &procedure_result.get_name();
            yield_items_string += " AS ";
            yield_items_string += &first_variable.get_name();
            yield_items.push((Some(procedure_result), first_variable));
        } else {
            yield_items_string += &first_variable.get_name();
            yield_items.push((None, first_variable));
        }

        // yield_item*
        for _ in 0..self.random.d2() {
            yield_items_string += ",";
            let variable = self.variables.new_variable();
            if self.random.bool() {
                let procedure_result = self.variables.get_procedure_result();
                yield_items_string += &procedure_result.get_name();
                yield_items_string += " AS ";
                yield_items_string += &variable.get_name();
                yield_items.push((Some(procedure_result), variable));
            } else {
                yield_items_string += &variable.get_name();
                yield_items.push((None, variable));
            }
        }

        // where_clause
        let where_clause = if self.random.bool() {
            let where_expression = Expression::new();
            yield_items_string += " WHERE ";
            yield_items_string += &where_expression.get_name();
            Some(where_expression)
        } else {
            None
        };

        (
            CypherNode::YieldItems {
                yield_items,
                where_clause,
            },
            yield_items_string,
        )
    }

    fn visit_reading_clause(&mut self) -> Self::Output {
        let (reading_clause, reading_string) = match self.random.d6() {
            0 => self.visit_match(),
            1 => self.visit_unwind(),
            2 => self.visit_in_query_call(),
            _ => {
                // todo: need to modify
                self.visit_match()
            }
        };

        (
            CypherNode::ReadingClause {
                reading_clause: Box::new(reading_clause),
            },
            reading_string,
        )
    }

    fn visit_match(&mut self) -> Self::Output {
        let mut match_string = String::new();

        let is_optional = if self.random.bool() {
            match_string += "OPTIONAL ";
            true
        } else {
            false
        };
        match_string += "MATCH ";

        let (pattern_node, pattern_string) = self.visit_pattern();
        let pattern = Box::new(pattern_node);
        match_string += &pattern_string;

        let where_clause = if self.random.bool() {
            let where_expression = Expression::new();
            match_string += " WHERE ";
            match_string += &where_expression.get_name();
            Some(where_expression)
        } else {
            None
        };

        (
            CypherNode::Match {
                is_optional,
                pattern,
                where_clause,
            },
            match_string,
        )
    }

    // unwind: UNWIND expression AS variable.
    fn visit_unwind(&mut self) -> Self::Output {
        let mut unwind_string = String::new();

        unwind_string += "UNWIND ";

        let expression = Expression::new();
        unwind_string += &expression.get_name();
        unwind_string += " AS ";

        let variable = self.variables.new_variable();
        unwind_string += &variable.get_name();

        (
            CypherNode::Unwind {
                expression,
                variable,
            },
            unwind_string,
        )
    }

    fn visit_updating_clause(&mut self) -> Self::Output {
        let (updating_clause, updating_string) = match self.random.d6() {
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

        (
            CypherNode::UpdatingClause {
                updating_clause: Box::new(updating_clause),
            },
            updating_string,
        )
    }

    fn visit_create(&mut self) -> Self::Output {
        let (pattern, pattern_string) = self.visit_pattern();

        (
            CypherNode::Create {
                pattern: Box::new(pattern),
            },
            pattern_string,
        )
    }

    // merge: MERGE pattern_part (merge_action)*; merge_action: on match|create set.
    fn visit_merge(&mut self) -> Self::Output {
        let mut merge_string = "Merge ".to_string();

        let (pattern_part_node, pattern_part_string) = self.visit_pattern_part();
        merge_string += &pattern_part_string;
        let pattern_part = Box::new(pattern_part_node);

        let mut merge_actions = Vec::new();

        for _ in 0..self.random.d2() {
            merge_string += " ";
            let (merge_action, merge_action_string) = self.visit_set();
            if self.random.bool() {
                // ON MATCH Set
                merge_string += "ON MATCH ";
            } else {
                // ON CREATE Set
                merge_string += "ON CREATE ";
            }
            merge_string += &merge_action_string;
            merge_actions.push(Box::new(merge_action));
        }

        (
            CypherNode::Merge {
                pattern_part,
                merge_actions,
            },
            merge_string,
        )
    }

    // delete: detach? delete Vec<expressions>
    fn visit_delete(&mut self) -> Self::Output {
        let mut delete_string = String::new();

        let is_detach = if self.random.bool() {
            delete_string += "DETACH DELETE ";
            true
        } else {
            delete_string += "DELETE ";
            false
        };

        // todo: need to modify: delete existing expression.
        let mut expressions = Vec::new();
        let expression = Expression::new();
        delete_string += &expression.get_name();
        expressions.push(expression);

        for _ in 0..self.random.d2() {
            let expression = Expression::new();
            delete_string += ",";
            delete_string += &expression.get_name();
            expressions.push(expression);
        }

        (
            CypherNode::Delete {
                is_detach,
                expressions,
            },
            delete_string,
        )
    }

    // set: set (property = Expression | Variable = Expression | Variable += Expression | Variable = NodeLabels)*
    fn visit_set(&mut self) -> Self::Output {
        let mut set_string = "SET ".to_string();
        let mut property_set = vec![];
        let mut variable_set = vec![];
        let mut variable_add = vec![];
        let mut label_set = vec![];

        // first set_item
        match self.random.d6() {
            0 => {
                let property = PropertyExpression::new();
                let expression = Expression::new();
                set_string += &property.get_name();
                set_string += "=";
                set_string += &expression.get_name();
                property_set.push((property, expression));
            }
            1 => {
                let variable = self.variables.get_old_variable();
                let expression = Expression::new();
                set_string += &variable.get_name();
                if self.random.bool() {
                    set_string += "=";
                    set_string += &expression.get_name();
                    variable_set.push((variable, expression));
                } else {
                    set_string += "+=";
                    set_string += &expression.get_name();
                    variable_add.push((variable, expression));
                }
            }
            2 => {
                let variable = self.variables.get_old_variable();
                set_string += &variable.get_name();
                set_string += " ";
                // NodeLabels: NodeLabel+
                let mut node_labels = vec![];
                let first_label = NodeLabel::new();
                set_string += ":";
                set_string += &first_label.get_name();
                node_labels.push(first_label);

                for _ in 0..self.random.d2() {
                    let node_label = NodeLabel::new();
                    set_string += " :";
                    set_string += &node_label.get_name();
                    node_labels.push(node_label);
                }
                label_set.push((variable, node_labels));
            }
            _ => {}
        }

        // todo: repeat above operator.
        for _ in 0..self.random.d2() {}

        (
            CypherNode::Set {
                property_set,
                variable_set,
                variable_add,
                label_set,
            },
            set_string,
        )
    }

    // remove: remove (variable Nodelabel* | PropertyExpression)+
    fn visit_remove(&mut self) -> Self::Output {
        let mut remove_string = "REMOVE ".to_string();
        let mut variable_remove = vec![];
        let mut property_remove = vec![];

        if self.random.bool() {
            let variable = self.variables.get_old_variable();
            remove_string += &variable.get_name();
            remove_string += " ";

            let mut node_labels = vec![];
            let first_label = NodeLabel::new();
            remove_string += ":";
            remove_string += &first_label.get_name();
            node_labels.push(first_label);

            for _ in 0..self.random.d2() {
                let node_label = NodeLabel::new();
                remove_string += " :";
                remove_string += &node_label.get_name();
                node_labels.push(node_label);
            }
            variable_remove.push((variable, node_labels));
        } else {
            let property_expression = PropertyExpression::new();
            remove_string += &property_expression.get_name();
            property_remove.push(property_expression);
        }

        // todo: repeat above operator.
        for _ in 0..self.random.d2() {}

        (
            CypherNode::Remove {
                variable_remove,
                property_remove,
            },
            remove_string,
        )
    }

    /// Return clause: return projection_body.
    fn visit_return(&mut self) -> Self::Output {
        let mut return_string = String::new();
        let (projection_body, projection_string) = self.visit_projection_body();
        return_string += "RETURN";
        return_string += &projection_string;
        (
            CypherNode::Return {
                projection_body: Box::new(projection_body),
            },
            return_string,
        )
    }

    fn visit_projection_body(&mut self) -> Self::Output {
        let mut projection_body_string = String::new();

        // DISTINCT
        let is_distinct = if self.random.bool() {
            projection_body_string += " DISTINCT";
            true
        } else {
            false
        };

        // ProjectionItems
        let (projection_items_node, projection_items_string) = self.visit_projection_items();
        projection_body_string += " ";
        projection_body_string += &projection_items_string;
        let projection_items = Box::new(projection_items_node);

        // order:
        let order = if self.random.low_prob_bool() {
            let (order_node, order_string) = self.visit_order();
            projection_body_string += " ";
            projection_body_string += &order_string;
            Some(Box::new(order_node))
        } else {
            None
        };
        let skip = if self.random.low_prob_bool() {
            let skip_expression = Expression::new();
            projection_body_string += " ";
            projection_body_string += &skip_expression.get_name();
            Some(skip_expression)
        } else {
            None
        };
        let limit = if self.random.low_prob_bool() {
            let limit_expression = Expression::new();
            projection_body_string += " ";
            projection_body_string += &limit_expression.get_name();
            Some(limit_expression)
        } else {
            None
        };
        (
            CypherNode::ProjectionBody {
                is_distinct,
                projection_items,
                order,
                skip,
                limit,
            },
            projection_body_string,
        )
    }

    fn visit_projection_items(&mut self) -> Self::Output {
        let mut projection_items_string = String::new();

        let mut expressions = Vec::new();
        let is_all = if self.random.bool() {
            projection_items_string += "*";
            true
        } else {
            let expression = Expression::new();
            projection_items_string += &expression.get_name();

            let var = if self.random.bool() {
                let variable = self.variables.new_variable();
                projection_items_string += " AS ";
                projection_items_string += &variable.get_name();
                Some(variable)
            } else {
                None
            };
            expressions.push((expression, var));
            false
        };

        // projection_items
        for _ in 0..self.random.d2() {
            projection_items_string += " , ";

            let expression = Expression::new();
            projection_items_string += &expression.get_name();

            let var = if self.random.bool() {
                let variable = self.variables.new_variable();
                projection_items_string += " AS ";
                projection_items_string += &variable.get_name();
                Some(variable)
            } else {
                None
            };
            expressions.push((expression, var));
        }

        (
            CypherNode::ProjectionItems {
                is_all,
                expressions,
            },
            projection_items_string,
        )
    }

    /// order: order by sort_items
    fn visit_order(&mut self) -> Self::Output {
        let mut order_string = "ORDER BY ".to_string();

        let sort_rules = vec!["ASC", "DESC", "ASCENDING", "DESCENDING"];
        let mut sort_items = vec![];

        let first_expression = Expression::new();
        order_string += &first_expression.get_name();

        let rule = if self.random.bool() {
            let rule_string = sort_rules[self.random.d2() as usize].to_string();
            order_string += " ";
            order_string += &rule_string;
            Some(rule_string)
        } else {
            None
        };
        sort_items.push((first_expression, rule));

        for _ in 0..self.random.d2() {
            let expression = Expression::new();
            order_string += &expression.get_name();

            let rule = if self.random.bool() {
                let rule_string = sort_rules[self.random.d2() as usize].to_string();
                order_string += " ";
                order_string += &rule_string;
                Some(rule_string)
            } else {
                None
            };
            sort_items.push((expression, rule))
        }

        (CypherNode::Order { sort_items }, order_string)
    }

    // Pattern: PatternPart+
    fn visit_pattern(&mut self) -> Self::Output {
        let mut pattern_string = String::new();
        let mut pattern_parts = vec![];

        let (pattern_part_node, pattern_part_string) = self.visit_pattern_part();
        pattern_string += &pattern_part_string;
        pattern_parts.push(Box::new(pattern_part_node));

        for _ in 0..self.random.d2() {
            let (pattern_part_node, pattern_part_string) = self.visit_pattern_part();
            pattern_string += " , ";
            pattern_string += &pattern_part_string;
            pattern_parts.push(Box::new(pattern_part_node));
        }

        (CypherNode::Pattern { pattern_parts }, pattern_string)
    }

    // PatternPart: (Variable =)? pattern_element
    fn visit_pattern_part(&mut self) -> Self::Output {
        let mut pattern_part_string = String::new();
        let var = if self.random.bool() {
            let variable = self.variables.new_variable();
            pattern_part_string += &variable.get_name();
            pattern_part_string += " = ";
            Some(variable)
        } else {
            None
        };

        let (pattern_element_node, pattern_element_string) = self.visit_pattern_element();
        pattern_part_string += &pattern_element_string;

        (
            CypherNode::PatternPart {
                var,
                pattern_element: Box::new(pattern_element_node),
            },
            pattern_part_string,
        )
    }

    // pattern_element: NodePattern (RelationshipPattern NodePattern)*
    fn visit_pattern_element(&mut self) -> Self::Output {
        let mut pattern_element_string = String::new();
        let parentheses_number = self.random.d2();

        // for _ in 0..parentheses_number {
        //     pattern_element_string += "(";
        // }

        let (node_pattern_node, node_pattern_string) = self.visit_node_pattern();
        let node_pattern = Box::new(node_pattern_node);
        pattern_element_string += &node_pattern_string;

        let mut pattern_element_chain = vec![];
        for _ in 0..self.random.d2() {
            pattern_element_string += " ";
            let (relationship_node, relationship_string) = self.visit_relationship_pattern();
            pattern_element_string += &relationship_string;

            pattern_element_string += " ";
            let (node, node_string) = self.visit_node_pattern();
            pattern_element_string += &node_string;

            pattern_element_chain.push((Box::new(relationship_node), Box::new(node)));
        }

        // for _ in 0..parentheses_number {
        //     pattern_element_string += ")";
        // }

        // let x = (0..parentheses_number).into_iter().map(|_| ")").collect::<String>();
        (
            CypherNode::PatternElement {
                parentheses: parentheses_number,
                pattern_element: (node_pattern, pattern_element_chain),
            },
            pattern_element_string,
        )
    }

    // NodePattern: ( Variable? (:label)* Properties)
    fn visit_node_pattern(&mut self) -> Self::Output {
        let mut node_pattern_string = "( ".to_string();

        let var = if self.random.bool() {
            let variable = self.variables.new_variable();
            node_pattern_string += &variable.get_name();
            Some(variable)
        } else {
            None
        };

        // use exists node label.
        // todo: in atlas graph, one vertex has only one vertex_label.
        let mut vertex_labels = vec![];
        // for _ in 0..self.random.d2() {
        // let node_label = NodeLabel::new();
        let node_label = self.graph_schema.rand_vertex_labels();
        let node_property = node_label.random_property();
        node_pattern_string += ":";
        node_pattern_string += &node_label.get_name();
        vertex_labels.push(node_label);
        // }

        // previous label's properties.

        let properties = if self.random.bool() {
            // let properties = Properties::new();
            let property_value = node_property.default_value();

            // node_pattern_string += &properties.get_name();

            // convert ast tree to string.
            node_pattern_string += "{";
            node_pattern_string += &node_property.get_name();
            node_pattern_string += ":";
            node_pattern_string += &property_value.to_string();
            node_pattern_string += "}";

            Some((node_property, property_value))
        } else {
            None
        };
        node_pattern_string += ")";

        (
            CypherNode::NodePattern {
                var,
                vertex_labels,
                properties,
            },
            node_pattern_string,
        )
    }

    // todo: need to modify.
    fn visit_relationship_pattern(&mut self) -> Self::Output {
        let mut relationship_pattern_string = String::new();

        let direction = match self.random.d6() {
            0 => RelationshipDirection::Left,
            1 => RelationshipDirection::Right,
            2 => RelationshipDirection::Both,
            3 => RelationshipDirection::None,
            _ => RelationshipDirection::None,
        };

        relationship_pattern_string += "<-[";

        let var = if self.random.bool() {
            let variable = self.variables.new_variable();
            relationship_pattern_string += &variable.get_name();
            Some(variable)
        } else {
            None
        };
        let mut edge_labels = vec![];
        if self.random.bool() {
            // let relation_label = NodeLabel::new();
            let relation_label = self.graph_schema.rand_edge_labels();
            relationship_pattern_string += ":";
            relationship_pattern_string += &relation_label.get_name();
            edge_labels.push(relation_label);

            for _ in 0..self.random.d2() {
                // let relation_label = NodeLabel::new();
                let relation_label = self.graph_schema.rand_edge_labels();
                relationship_pattern_string += "|:";
                relationship_pattern_string += &relation_label.get_name();
                edge_labels.push(relation_label);
            }
        }

        let range = if self.random.bool() {
            let range_start = self.random.d2();
            relationship_pattern_string += "*";
            relationship_pattern_string += &range_start.to_string();
            relationship_pattern_string += "..";

            let range_end = if self.random.bool() {
                let range_end = self.random.d6();
                relationship_pattern_string += &range_end.to_string();
                Some(range_end)
            } else {
                None
            };

            (Some(range_start), range_end)
        } else {
            (None, None)
        };

        let properties = if self.random.bool() {
            // let properties = Properties::new();
            // relationship_pattern_string += " ";
            // relationship_pattern_string += &properties.get_name();
            // Some(properties)
            let edge_property = edge_labels[0].clone().random_property();
            let property_value = edge_property.default_value();
            relationship_pattern_string += "{";
            relationship_pattern_string += &edge_property.get_name();
            relationship_pattern_string += ":";
            relationship_pattern_string += &property_value.to_string();
            relationship_pattern_string += "}";

            Some((edge_property, property_value))
        } else {
            None
        };

        relationship_pattern_string += "]->";

        (
            CypherNode::RelationshipPattern {
                direction,
                var,
                edge_labels,
                range,
                properties,
            },
            relationship_pattern_string,
        )
    }

    // PropertyOrLabelsExpression: use to generate exprssion
    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        let mut expression_string = String::new();
        // atom: Literal
        let (atom_node, atom_string) = self.visit_atom();
        expression_string += &atom_string;

        let mut property_lookups = vec![];
        // PropertyLookUp*
        for _ in 0..self.random.d2() {
            expression_string += " .";
            let property_lookup = SchemaName::new(&mut self.random);
            expression_string += &property_lookup.get_name();
            property_lookups.push(property_lookup);
        }

        let mut node_labels = vec![];
        for _ in 0..self.random.d2() {
            let node_label = NodeLabel::new();
            expression_string += " :";
            expression_string += &node_label.get_name();
            node_labels.push(node_label);
        }

        (
            CypherNode::PropertyOrLabelsExpression {
                atom: Box::new(atom_node),
                property_lookups,
                node_labels,
            },
            expression_string,
        )
    }

    fn visit_atom(&mut self) -> Self::Output {
        let mut atom_string = String::new();

        let (literal, expressions, sub_expression, is_variable) = match self.random.d20() % 15 {
            // Literal
            0 => {
                let literal = Literal::Integer(1);
                (Some(literal), vec![], None, None)
            }
            // Parameter: $ SymbolicName|Integer
            1 => {
                atom_string += "$";
                let variable = self.variables.get_symbolic_or_integer();
                atom_string += &variable.get_name();
                (None, vec![], None, Some(variable))
            }
            // CaseExpression: CASE expression? (WHEN expression THEN expression)+ (ELSE expression)? END
            2 => {
                atom_string += "CASE";
                let mut expressions = vec![];
                if self.random.bool() {
                    let expression = Expression::new();
                    atom_string += " ";
                    atom_string += &expression.get_name();
                    expressions.push(expression)
                }

                for _ in 0..(self.random.d2() + 1) {
                    atom_string += " WHEN ";
                    let when_expression = Expression::new();
                    atom_string += &when_expression.get_name();
                    let then_expression = Expression::new();
                    atom_string += " THEN ";
                    atom_string += &then_expression.get_name();
                    expressions.push(when_expression);
                    expressions.push(then_expression);
                }

                if self.random.bool() {
                    let expression = Expression::new();
                    atom_string += "ELSE ";
                    atom_string += &expression.get_name();
                    expressions.push(expression)
                }

                atom_string += " END";
                (None, expressions, None, None)
            }
            // `COUNT`
            3 => {
                atom_string += "COUNT (*)";
                (None, vec![], None, None)
            }
            // ListComprehension: [FilterExpression (| Expression)?]
            4 => {
                atom_string += "[";
                let mut expressions = vec![];

                let (filter_node, filter_string) = self.visit_filter_expression();
                atom_string += &filter_string;

                if self.random.bool() {
                    atom_string += "|";
                    let expression = Expression::new();
                    atom_string += &expression.get_name();
                    expressions.push(expression)
                }

                (None, expressions, Some(Box::new(filter_node)), None)
            }
            // PatternComprehension: [ (Variable =)? RelationshipsPattern (where Expression)? | Expression]
            5 => {
                atom_string += "[";
                // true = is variable
                let variable = if self.random.bool() {
                    let is_variable = self.variables.new_variable();
                    atom_string += &is_variable.get_name();
                    atom_string += "=";
                    Some(is_variable)
                } else {
                    None
                };
                let (relationships_node, relationships_string) = self.visit_relationships_pattern();
                atom_string += &relationships_string;

                atom_string += " ";

                let mut expressions = vec![];
                // is where clause
                if self.random.bool() {
                    let where_expression = Expression::new();
                    atom_string += "WHERE ";
                    atom_string += &where_expression.get_name();
                    expressions.push(where_expression)
                }
                atom_string += "| ";
                let expression = Expression::new();
                atom_string += &expression.get_name();
                atom_string += "]";
                (
                    None,
                    expressions,
                    Some(Box::new(relationships_node)),
                    variable,
                )
            }
            // (ALL|ANY|NONE|SINGLE) ( FilterExpression )
            6 | 7 | 8 | 9 => {
                let rel = vec!["ALL", "ANY", "SINGLE", "NONE"];
                let (filter_expression, filter_expression_string) = self.visit_filter_expression();
                atom_string += rel[(self.random.d6() % 3) as usize];
                atom_string += "(";
                atom_string += &filter_expression_string;
                atom_string += ")";

                (None, vec![], Some(Box::new(filter_expression)), None)
            }
            // RelationshipsPattern： NodePattern (RelationShipPattern, NodePattern)*
            10 => {
                let (relationships_node, relationships_string) = self.visit_relationships_pattern();
                atom_string += &relationships_string;
                (None, vec![], Some(Box::new(relationships_node)), None)
            }
            // ParenthesizedExpression: ( Expression )
            11 => {
                let expression = Expression::new();
                atom_string += "(";
                atom_string += &expression.get_name();
                atom_string += ")";

                (None, vec![expression], None, None)
            }
            // FunctionInvocation
            12 => {
                let (function_node, function_string) = self.visit_function_invocation();
                atom_string += &function_string;
                (None, vec![], Some(Box::new(function_node)), None)
            }
            // ExistentialSubquery
            13 => {
                atom_string += "EXISTS {";
                if self.random.bool() {
                    let (regular_node, regular_string) = self.visit_regular_query();
                    atom_string += &regular_string;
                    atom_string += "}";
                    (None, vec![], Some(Box::new(regular_node)), None)
                } else {
                    let (pattern_node, pattern_string) = self.visit_pattern();
                    let where_expression = Expression::new();
                    atom_string += &pattern_string;
                    atom_string += " WHERE ";
                    atom_string += &where_expression.get_name();
                    atom_string += "}";
                    (
                        None,
                        vec![where_expression],
                        Some(Box::new(pattern_node)),
                        None,
                    )
                }
            }
            // Variable
            14 => {
                let variable = self.variables.get_old_variable();
                atom_string += &variable.get_name();
                (None, vec![], None, Some(variable))
            }
            _ => (None, vec![], None, None),
        };

        (
            CypherNode::Atom {
                literal,
                expressions,
                sub_expression,
                is_variable,
            },
            atom_string,
        )
    }

    // filter expression.
    fn visit_filter_expression(&mut self) -> Self::Output {
        let mut filter_expression_string = String::new();
        let variable = self.variables.new_variable();
        let expression = Expression::new();
        filter_expression_string += &variable.get_name();
        filter_expression_string += " IN ";
        filter_expression_string += &expression.get_name();

        let where_clause = if self.random.bool() {
            let expr = Expression::new();
            filter_expression_string += " WHERE ";
            filter_expression_string += &expr.get_name();
            Some(expr)
        } else {
            None
        };

        (
            CypherNode::FilterExpression {
                id_in_coll: (variable, expression),
                where_clause,
            },
            filter_expression_string,
        )
    }

    // RelationShipsPattern
    fn visit_relationships_pattern(&mut self) -> Self::Output {
        let mut relationships_string = String::new();

        let (node_pattern_node, node_pattern_string) = self.visit_node_pattern();
        let node_pattern = Box::new(node_pattern_node);
        relationships_string += &node_pattern_string;

        let mut pattern_element_chain = vec![];
        for _ in 0..(self.random.d2() + 1) {
            relationships_string += " ";
            let (relationship_node, relationship_string) = self.visit_relationship_pattern();
            relationships_string += &relationship_string;

            relationships_string += " ";
            let (node, node_string) = self.visit_node_pattern();
            relationships_string += &node_string;

            pattern_element_chain.push((Box::new(relationship_node), Box::new(node)));
        }

        (
            CypherNode::RelationshipsPattern {
                node_pattern,
                pattern_element_chain,
            },
            relationships_string,
        )
    }

    // FunctionInvocation
    fn visit_function_invocation(&mut self) -> Self::Output {
        let mut function_string = String::new();

        let is_exists = if self.random.bool() {
            function_string += "EXISTS";
            (true, None)
        } else {
            // todo: need to modify because namespace is *
            let name_space = NameSpace::new();
            let variable = self.variables.get_procedure_method();
            function_string += &name_space.get_name();
            function_string += ".";
            function_string += &variable.get_name();

            (false, Some((name_space, variable)))
        };

        function_string += "(";
        let is_distinct = if self.random.bool() {
            function_string += "DISTINCT ";
            true
        } else {
            false
        };

        let mut expressions = vec![];
        if self.random.bool() {
            let expression = Expression::new();
            function_string += &expression.get_name();
            expressions.push(expression);
            for _ in 0..self.random.d2() {
                let expression = Expression::new();
                function_string += ", ";
                function_string += &expression.get_name();
                expressions.push(expression);
            }
        }
        function_string += ")";

        (
            CypherNode::FunctionInvocation {
                is_exists,
                is_distinct,
                expressions,
            },
            function_string,
        )
    }
}
