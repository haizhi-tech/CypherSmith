use super::cypher_gen_new::CypherGenerator;
use super::{constants, ExpressionNodeVisitor};
use crate::common::{
    BinOpKind, CaseAlternative, CmpKind, Expr, ExprKind, Literal, PredicateFunctionKind,
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
        self.complexity = 0;
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

    // todo: need implement
    pub fn random_literal(&mut self) -> Literal {
        Literal::Integer(1)
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

        if (self.random.d9() == 1) && (self.complexity < self.limit) {
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

    /// todo: useless, delete
    fn visit_partial_comparison_expression(&mut self) -> Self::Output {
        todo!()
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
        while self.random.d20() == 1 {
            if self.random.d6() == 1 {
                // StringOperatorExpression
                let string_expr = self.visit_property_or_labels_expression();
                let kind = ExprKind::BinOp(
                    self.random_string_kind(),
                    Box::new(query_expr),
                    Box::new(string_expr),
                );
                query_expr = Expr::from(kind);
            } else if self.random.d6() == 1 {
                // ListOperatorExpression: In | [Expression] | [Expression..Expression]
                if self.random.d6() > 2 {
                    // In PropertyOrLabelsExpression
                    let list_expr = self.visit_property_or_labels_expression();
                    let kind =
                        ExprKind::BinOp(BinOpKind::In, Box::new(query_expr), Box::new(list_expr));
                    query_expr = Expr::from(kind);
                } else if self.random.d6() == 1 {
                    // [Expression]
                    let list_expr = self.visit();
                    let kind = ExprKind::BinOp(
                        BinOpKind::Index,
                        Box::new(query_expr),
                        Box::new(list_expr),
                    );
                    query_expr = Expr::from(kind);
                } else if self.random.d6() == 1 {
                    // todo: [(Expression)?..(Expression)?]
                }
            } else if self.random.d6() == 1 {
                // NullOperatorExpression
                let kind = ExprKind::UnOp(self.random_null_kind(), Box::new(query_expr));
                query_expr = Expr::from(kind);
            }
        }

        query_expr
    }

    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        // let mut pro_expr = self.visit_atom();
        // pro_expr
        todo!()
    }

    /// Atom: Literal | Parameter | Case Expression | COUNT (*)
    fn visit_atom(&mut self) -> Self::Output {
        let select_number = self.random.d20();

        match select_number {
            // Literal Expression
            0 => {
                self.complexity += 1;
                Expr::from(ExprKind::Lit(self.random_literal()))
            }
            // CaseExpression
            1 => {
                let case_expr = if self.random.d6() == 1 {
                    self.complexity += 1;
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
                    self.complexity += 1;
                    Some(Box::new(self.visit()))
                } else {
                    None
                };

                Expr::from(ExprKind::Case(case_expr, case_alternatives, else_expr))
            }
            // COUNT (*)
            2 => Expr::from(ExprKind::Lit(Literal::String("COUNT (*)".to_string()))),
            // ListComprehension
            3 => {
                todo!()
            }
            // PatternComprehension
            4 => {
                todo!()
            }
            // ALL|ANY|NONE|SINGLE ( FilterExpression )
            // FilterExpression: Variable IN Expression WhereExpression
            5 => {
                let var = self.cypher.variables.get_old_variable();
                let expression = self.visit();
                let where_expression = self.visit();
                let kind = ExprKind::PredicateFunction(
                    self.random_predicate_function_kind(),
                    var,
                    Box::new(expression),
                    Box::new(where_expression),
                );
                Expr::from(kind)
            }
            // RelationShipsPattern
            6 => {
                todo!()
            }
            // ParenthesizedExpression
            7 => {
                let expression = self.visit();
                Expr::from(ExprKind::UnOp(UnOpKind::Parentheses, Box::new(expression)))
            }
            // FunctionInvocation: FunctionName ( (DISTINCT)? Expression*)
            8 => {
                // FunctionName: Namespace.SymbolicName
                // todo: need to implement NameSpace and SymbolicName
                let function = Expr::from(ExprKind::Lit(Literal::String(
                    "atlas.shortestpath".to_string(),
                )));

                let is_distinct = self.random.bool();

                // Vec<Expression>
                let mut vec_expr = Vec::new();
                for _ in 0..self.random.d2() {
                    vec_expr.push(self.visit());
                }

                Expr::from(ExprKind::Invocation(
                    Box::new(function),
                    is_distinct,
                    vec_expr,
                ))
            }
            // ExistentialSubquery
            9 => {
                // ExistentialSubquery: `EXISTS` `{` (RegularQuery|(Pattern where)) `}`
                let query = self.cypher.visit();
                Expr::from(ExprKind::SubQuery(SubQueryKind::Exists, Box::new(query)))
            }
            // Variable
            10 => {
                let var = self.cypher.variables.get_old_variable();
                Expr::from(ExprKind::Variable(var))
            }
            _ => {
                self.complexity += 1;
                Expr::from(ExprKind::Lit(self.random_literal()))
            }
        }
    }
}