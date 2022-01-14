use super::{constants, cypher_gen::CypherGenerator, ExpressionNodeVisitor};
use crate::common::{
    BinOpKind, CaseAlternative, CmpKind, DataKind, Expr, ExprKind, Literal, PredicateFunctionKind,
    RandomGenerator, SubQueryKind, UnOpKind,
};

pub struct ExprGenerator<'a> {
    random: RandomGenerator,
    cypher: &'a mut CypherGenerator,
    complexity: i32,
    limit: i32,
    loop_limit: i32,
}

impl<'a> ExprGenerator<'a> {
    pub fn new(cypher: &'a mut CypherGenerator) -> ExprGenerator<'a> {
        cypher.limit -= constants::DEFAULT_EXPRESSION_LIMIT;
        ExprGenerator {
            random: RandomGenerator::new(),
            cypher,
            complexity: 0,
            limit: constants::DEFAULT_EXPRESSION_LIMIT,
            loop_limit: constants::DEFAULT_LOOP_LIMIT,
        }
    }
}

impl ExprGenerator<'_> {
    pub fn visit(&mut self) -> Expr {
        // self.complexity = 0;
        self.visit_expression()
    }
}

impl ExprGenerator<'_> {
    pub fn random_cmp_kind(&mut self) -> CmpKind {
        let cmp_kinds = [
            CmpKind::Ne,
            CmpKind::Eq,
            CmpKind::Lt,
            CmpKind::Gt,
            CmpKind::Le,
            CmpKind::Ge,
        ];
        cmp_kinds[self.random.d6() as usize % 6]
    }

    pub fn random_add_or_sub_kind(&mut self) -> BinOpKind {
        let kinds = [BinOpKind::Add, BinOpKind::Sub];
        kinds[self.random.d2() as usize % 2]
    }

    pub fn random_mul_div_mod_kind(&mut self) -> BinOpKind {
        let kinds = [BinOpKind::Mul, BinOpKind::Div, BinOpKind::Mod];
        kinds[self.random.d6() as usize % 3]
    }

    pub fn random_unary_kind(&mut self) -> UnOpKind {
        let kinds = [UnOpKind::Pos, UnOpKind::Neg];
        kinds[self.random.d2() as usize % 2]
    }

    pub fn random_string_kind(&mut self) -> BinOpKind {
        let kinds = [
            BinOpKind::StartsWith,
            BinOpKind::EndsWith,
            BinOpKind::Contains,
        ];
        kinds[self.random.d6() as usize % 3]
    }

    pub fn random_null_kind(&mut self) -> UnOpKind {
        let kinds = [UnOpKind::Null, UnOpKind::NotNull];
        kinds[self.random.d2() as usize % 2]
    }

    // Random Literal Gen.
    pub fn random_literal(&mut self) -> Literal {
        if self.random.bool() {
            Literal::Integer(self.random.under(5) as u64)
        } else if self.random.bool() {
            Literal::Boolean(self.random.bool())
        } else if self.random.bool() {
            Literal::Double(self.random.under(5) as f64)
        } else if self.random.d6() == 1 {
            Literal::String(self.random.under(5).to_string())
        } else if self.random.d12() == 1 {
            let expr = Expr::from(ExprKind::Lit(Literal::Integer(self.random.under(5) as u64)));
            Literal::List(vec![expr])
        } else if self.random.d20() == 1 {
            let random_number = self.random.under(5) as u64;
            let expr = Expr::from(ExprKind::Lit(Literal::Integer(random_number)));
            Literal::Map(vec![(random_number.to_string(), expr)])
        } else {
            Literal::Null
        }
    }

    pub fn random_predicate_function_kind(&mut self) -> PredicateFunctionKind {
        let kinds = [
            PredicateFunctionKind::All,
            PredicateFunctionKind::Any,
            PredicateFunctionKind::None,
            PredicateFunctionKind::Single,
        ];
        kinds[self.random.d6() as usize % 4]
    }
}

impl ExpressionNodeVisitor for ExprGenerator<'_> {
    type Output = Expr;

    /// ### Synopsis
    /// Expression: OrExpression
    fn visit_expression(&mut self) -> Self::Output {
        self.visit_or_expression()
    }

    /// ### Synopsis
    /// OrExpression: XorExpression (OR XorExpression)*
    fn visit_or_expression(&mut self) -> Self::Output {
        let mut or_expr = self.visit_xor_expression();

        // random loop
        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new or clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_xor_expression();
                // new Expression.
                let kind = ExprKind::BinOp(BinOpKind::Or, Box::new(or_expr), Box::new(rhs));
                or_expr = Expr::from(kind);
            }
        }

        or_expr
    }

    /// ### Synopsis
    /// XorExpression: AndExpression (XOR AndExpression)*
    fn visit_xor_expression(&mut self) -> Self::Output {
        let mut xor_expr = self.visit_and_expression();

        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new xor clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_xor_expression();
                // new Expression.
                let kind = ExprKind::BinOp(BinOpKind::Xor, Box::new(xor_expr), Box::new(rhs));
                xor_expr = Expr::from(kind);
            }
        }

        xor_expr
    }

    /// ### Synopsis
    /// AndExpression: NotExpression (AND NotExpression)*
    fn visit_and_expression(&mut self) -> Self::Output {
        let mut and_expr = self.visit_not_expression();

        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new and clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_not_expression();
                // new Expression.
                let kind = ExprKind::BinOp(BinOpKind::And, Box::new(and_expr), Box::new(rhs));
                and_expr = Expr::from(kind);
            }
        }

        and_expr
    }

    /// ### Synopsis
    /// NotExpression: Not? ComparsionExpression
    fn visit_not_expression(&mut self) -> Self::Output {
        let mut not_expr = self.visit_comparison_expression();

        if (self.random.d12() == 1) && (self.complexity < self.limit) {
            // new not clause.
            self.complexity += 1;
            // new not expression.
            let kind = ExprKind::UnOp(UnOpKind::Not, Box::new(not_expr));
            not_expr = Expr::from(kind);
        }

        not_expr
    }

    /// ### Synopsis
    /// ComparisonExpression: AddOrSubtractExpression (PartialComparisonExpression)*;
    fn visit_comparison_expression(&mut self) -> Self::Output {
        let mut cmp_expr = self.visit_add_or_subtract_expression();
        let mut tails = Vec::new();

        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new cmp clause, increase complexity.
                self.complexity += 1;
                let kind = self.random_cmp_kind();
                let rhs = self.visit_add_or_subtract_expression();
                tails.push((kind, Box::new(rhs)));
            }
        }
        if !tails.is_empty() {
            let kind = ExprKind::Cmp(Box::new(cmp_expr), tails);
            cmp_expr = Expr::from(kind);
        }
        cmp_expr
    }

    /// ### Synopsis
    /// AddOrSubtractExpression: MultiplyDivideModuloExpression (+/-  MultiplyDivideModuloExpression)*
    fn visit_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret_expr = self.visit_multiply_divide_modulo_expression();

        // random loop
        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new add/subtract clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_multiply_divide_modulo_expression();
                // new Expression.
                let kind = ExprKind::BinOp(
                    self.random_add_or_sub_kind(),
                    Box::new(ret_expr),
                    Box::new(rhs),
                );
                ret_expr = Expr::from(kind);
            }
        }

        ret_expr
    }

    /// ### Synopsis
    /// MultiplyDivideModuloExpression: PowerOfExpression (*/%// PowerOfExpression)*
    fn visit_multiply_divide_modulo_expression(&mut self) -> Self::Output {
        let mut ret_expr = self.visit_power_of_expression();

        // random loop
        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new *///% clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_power_of_expression();
                // new Expression.
                let kind = ExprKind::BinOp(
                    self.random_mul_div_mod_kind(),
                    Box::new(ret_expr),
                    Box::new(rhs),
                );
                ret_expr = Expr::from(kind);
            }
        }

        ret_expr
    }

    /// ### Synopsis
    /// PowerOfExpression: UnaryAddOrSubtractExpression ('^', UnaryAddOrSubtractExpression)* ;
    fn visit_power_of_expression(&mut self) -> Self::Output {
        let mut power_expr = self.visit_unary_add_or_subtract_expression();

        // random loop
        for _ in 0..self.loop_limit {
            // complexity limit.
            if (self.random.d20() == 1) && (self.complexity < self.limit) {
                // new power clause, increase complexity.
                self.complexity += 1;
                let rhs = self.visit_unary_add_or_subtract_expression();
                // new Expression.
                let kind = ExprKind::BinOp(BinOpKind::Pow, Box::new(power_expr), Box::new(rhs));
                power_expr = Expr::from(kind);
            }
        }

        power_expr
    }

    /// ### Synopsis
    /// UnaryAddOrSubtractExpression: ('+'|'-')? StringListNullOperatorExpression ;
    fn visit_unary_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut unary_expr = self.visit_string_list_null_operator_expression();

        if (self.random.d9() == 1) && (self.complexity < self.limit) {
            // new not clause.
            self.complexity += 1;
            // new not expression.
            let kind = ExprKind::UnOp(self.random_unary_kind(), Box::new(unary_expr));
            unary_expr = Expr::from(kind);
        }

        unary_expr
    }

    /// ### Synopsis
    /// StringListNullOperatorExpression: PropertyOrLabelsExpression (String|List|NullExpression)*
    fn visit_string_list_null_operator_expression(&mut self) -> Self::Output {
        let mut query_expr = self.visit_property_or_labels_expression();

        // expr loop
        for _ in 0..self.random.under(3) {
            if self.random.d20() == 1 {
                if (self.complexity < self.limit) && (self.random.d6() == 1) {
                    // StringOperatorExpression
                    self.complexity += 1;
                    let string_expr = self.visit_property_or_labels_expression();
                    let kind = ExprKind::BinOp(
                        self.random_string_kind(),
                        Box::new(query_expr),
                        Box::new(string_expr),
                    );
                    query_expr = Expr::from(kind);
                } else if (self.complexity < self.limit) && (self.random.d6() == 1) {
                    // ListOperatorExpression: In | [Expression] | [Expression..Expression]
                    if self.random.d6() > 2 {
                        // In PropertyOrLabelsExpression
                        self.complexity += 1;
                        let list_expr = self.visit_property_or_labels_expression();
                        let kind = ExprKind::BinOp(
                            BinOpKind::In,
                            Box::new(query_expr),
                            Box::new(list_expr),
                        );
                        query_expr = Expr::from(kind);
                    } else if self.random.d6() == 1 {
                        // [Expression]
                        self.complexity += 1;
                        let list_expr = self.visit();
                        let kind = ExprKind::BinOp(
                            BinOpKind::Index,
                            Box::new(query_expr),
                            Box::new(list_expr),
                        );
                        query_expr = Expr::from(kind);
                    } else if self.random.d12() == 1 {
                        // [(Expression)?..(Expression)?]
                        self.complexity += 1;
                        let (start_expr, end_expr) = if self.random.bool() {
                            let start_number = self.random.d2();
                            if self.random.bool() {
                                let end_number = start_number + self.random.d6();
                                (
                                    Expr::from(ExprKind::Lit(Literal::Integer(
                                        start_number as u64,
                                    ))),
                                    Expr::from(ExprKind::Lit(Literal::Integer(end_number as u64))),
                                )
                            } else {
                                (
                                    Expr::from(ExprKind::Lit(Literal::Integer(
                                        start_number as u64,
                                    ))),
                                    Expr::from(ExprKind::Lit(Literal::NullValue)),
                                )
                            }
                        } else if self.random.bool() {
                            let end_number = self.random.d6();
                            (
                                Expr::from(ExprKind::Lit(Literal::NullValue)),
                                Expr::from(ExprKind::Lit(Literal::Integer(end_number as u64))),
                            )
                        } else {
                            (
                                Expr::from(ExprKind::Lit(Literal::NullValue)),
                                Expr::from(ExprKind::Lit(Literal::NullValue)),
                            )
                        };

                        let list_expr = Expr::from(ExprKind::BinOp(
                            BinOpKind::Range,
                            Box::new(start_expr),
                            Box::new(end_expr),
                        ));

                        let kind = ExprKind::BinOp(
                            BinOpKind::Index,
                            Box::new(query_expr),
                            Box::new(list_expr),
                        );
                        query_expr = Expr::from(kind);
                    }
                } else if (self.complexity < self.limit) && (self.random.d6() == 1) {
                    // NullOperatorExpression
                    self.complexity += 1;
                    let kind = ExprKind::UnOp(self.random_null_kind(), Box::new(query_expr));
                    query_expr = Expr::from(kind);
                }
            }
        }

        query_expr
    }

    /// PropertyOrLabelsExpression
    ///
    /// Atom {PropertyLookup}* NodeLabel*
    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        // Property
        let mut query_expr = self.visit_atom();

        let data_kind = query_expr.kind.get_kind();

        if data_kind != DataKind::Vertex {
            return query_expr;
        }

        if (self.complexity < self.limit) && self.random.bool() {
            // PropertyLookup*
            for _ in 0..self.random.under(3) {
                if self.random.d12() == 1 {
                    let property = self
                        .cypher
                        .graph_schema
                        .random_vertex_property(&mut self.random);
                    if let Some(prop) = property {
                        query_expr = Expr::from(ExprKind::Property(Box::new(query_expr), prop));
                    }
                }
            }
        } else if (self.complexity < self.limit)
            && self.random.bool()
            && (query_expr.kind.get_kind() == DataKind::Vertex)
        {
            // Nodelabels
            for _ in 0..self.random.under(3) {
                if self.random.d12() == 1 {
                    let node_label = self.cypher.graph_schema.rand_vertex_label(&mut self.random);
                    query_expr = Expr::from(ExprKind::Label(Box::new(query_expr), node_label));
                }
            }
        }

        query_expr
    }

    /// Atom
    ///
    /// Literal | Parameter | Case Expression | COUNT (*)
    fn visit_atom(&mut self) -> Self::Output {
        let select_number = self.random.d100();

        match select_number {
            // Literal Expression
            0..=10 => Expr::from(ExprKind::Lit(self.random_literal())),
            // CaseExpression
            11..=13 => {
                self.complexity += 1;
                let case_expr = if self.random.d6() == 1 {
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                let mut case_alternatives = Vec::new();

                // WHEN expression THEN expression.
                for _ in 0..self.random.d2() + 1 {
                    case_alternatives.push(CaseAlternative {
                        condition: Box::new(self.visit()),
                        value: Box::new(self.visit()),
                    })
                }

                let else_expr = if self.random.d6() == 1 {
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                Expr::from(ExprKind::Case(case_expr, case_alternatives, else_expr))
            }
            // COUNT (*)
            14..=20 => Expr::from(ExprKind::Lit(Literal::String("COUNT (*)".to_string()))),
            // ListComprehension: [FilterExpression (|Expression)? ]
            21..=23 => {
                self.complexity += 1;

                let in_expression = self.visit();
                let var = self.cypher.variables.new_tmp_variable();
                //.new_kind_variable(in_expression.kind.get_kind());
                let in_expr = Box::new(in_expression);
                let where_expr = if self.random.d12() == 1 {
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                let mut filter_expr =
                    Expr::from(ExprKind::FilterExpression(var, in_expr, where_expr));

                if self.random.d20() == 1 {
                    self.complexity += 1;
                    filter_expr = Expr::from(ExprKind::BinOp(
                        BinOpKind::Pipe,
                        Box::new(filter_expr),
                        Box::new(self.visit()),
                    ));
                }

                Expr::from(ExprKind::Lit(Literal::List(vec![filter_expr])))
            }
            // PatternComprehension: [(variable =)? RelationShipsPattern (Where)? | Expression]
            24..=29 => {
                self.complexity += 1;

                let where_clause = if self.random.d20() == 1 {
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                let lhs = Expr::from(ExprKind::SubQuery(
                    SubQueryKind::PredicatePattern,
                    Box::new(self.cypher.expr_pattern()),
                    where_clause,
                ));
                let rhs = self.visit();

                let list_expr = Expr::from(ExprKind::BinOp(
                    BinOpKind::Pipe,
                    Box::new(lhs),
                    Box::new(rhs),
                ));

                Expr::from(ExprKind::Lit(Literal::List(vec![list_expr])))
            }
            // ALL|ANY|NONE|SINGLE (FilterExpression)
            // FilterExpression: Variable IN Expression (Where Expression)?
            30..=35 => {
                self.complexity += 1;

                let in_expression = self.visit();
                let var = self.cypher.variables.new_tmp_variable();
                // .new_kind_variable(in_expression.kind.get_kind());
                let in_expr = Box::new(in_expression);
                let where_expr = if self.random.d12() == 1 {
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                let filter_expr = Expr::from(ExprKind::FilterExpression(var, in_expr, where_expr));

                let kind = ExprKind::PredicateFunction(
                    self.random_predicate_function_kind(),
                    Box::new(filter_expr),
                );

                Expr::from(kind)
            }
            // RelationShipsPattern
            36..=40 => {
                self.complexity += 1;

                let pattern_query = self.cypher.expr_relation_pattern();
                Expr::from(ExprKind::SubQuery(
                    SubQueryKind::RelationShipsPattern,
                    Box::new(pattern_query),
                    None,
                ))
            }
            // ParenthesizedExpression
            41..=43 => {
                let expression = self.visit();
                Expr::from(ExprKind::UnOp(UnOpKind::Parentheses, Box::new(expression)))
            }
            // TODO: FunctionInvocation: FunctionName ( (DISTINCT)? Expression*)
            // 8 => {
            //     self.complexity += 1;

            //     // FunctionName: Namespace.SymbolicName
            //     let function = Expr::from(ExprKind::Lit(Literal::String(
            //         "atlas.shortestpath".to_string(),
            //     )));

            //     let is_distinct = self.random.bool();

            //     // Vec<Expression>
            //     let mut vec_expr = Vec::new();
            //     for _ in 0..self.random.d2() {
            //         vec_expr.push(self.visit());
            //     }

            //     Expr::from(ExprKind::Invocation(
            //         Box::new(function),
            //         is_distinct,
            //         vec_expr,
            //     ))
            // }
            // ExistentialSubquery
            46..=49 => {
                self.complexity += 1;

                // ExistentialSubquery: `EXISTS` `{` (RegularQuery|(Pattern where)) `}`
                let query = self.cypher.exec();
                Expr::from(ExprKind::SubQuery(
                    SubQueryKind::Exists,
                    Box::new(query),
                    None,
                ))
            }
            // Variable
            70..=99 => {
                let var = self.cypher.variables.get_old_variable();
                Expr::from(ExprKind::Variable(var))
            }
            _ => Expr::from(ExprKind::Lit(self.random_literal())),
        }
    }
}
