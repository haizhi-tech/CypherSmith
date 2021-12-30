use super::{constants, CypherGenerator, ExpressionNodeVisitor};
use crate::common::{Expr, RandomGenerator};

pub struct ExprGenerator<'a> {
    random: RandomGenerator,
    cypher: &'a mut CypherGenerator,
    complexity: i32,
    limit: i32,
}

impl<'a> ExprGenerator<'a> {
    pub fn new(cypher: &'a mut CypherGenerator) -> ExprGenerator<'a> {
        //
        cypher.limit -= constants::DEFAULT_EXPRESSION_LIMIT;
        ExprGenerator {
            random: RandomGenerator::new(),
            cypher,
            complexity: 0,
            limit: constants::DEFAULT_EXPRESSION_LIMIT,
        }
    }
}

impl ExprGenerator<'_> {
    pub fn visit(&mut self) -> Expr {
        self.complexity = 0;
        self.visit_expression()
    }
}

impl ExpressionNodeVisitor for ExprGenerator<'_> {
    type Output = Expr;

    fn visit_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_or_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_xor_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_and_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_not_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_comparison_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_partial_comparison_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_add_or_subtract_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_multiply_divide_modulo_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_power_of_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_unary_add_or_subtract_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_string_list_null_operator_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_string_operator_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_list_operator_expression(&mut self) -> Self::Output {
        todo!()
    }

    fn visit_null_operator_expression(&mut self) -> Self::Output {
        todo!()
    }
}
