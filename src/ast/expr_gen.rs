use super::{CypherGenerator, ExpressionNodeVisitor};
use crate::common::{Expression, RandomGenerator};

pub struct ExprGenerator<'a> {
    random: RandomGenerator,
    cypher: &'a mut CypherGenerator,
    complexity: usize,
    limit: usize,
}

impl<'a> ExprGenerator<'a> {
    pub fn new(cypher: &'a mut CypherGenerator) -> ExprGenerator<'a> {
        ExprGenerator {
            random: RandomGenerator::new(),
            cypher,
            complexity: 0,
            limit: 5usize,
        }
    }
}

impl ExprGenerator<'_> {
    pub fn visit(&mut self) -> (String, Expression) {
        let expression = self.visit_expression();
        (expression.clone(), Expression::from(expression))
    }

    // pub fn get_name(&self) -> String {
    //     self.expr_string.clone()
    // }

    pub fn visit_atom(&mut self) -> String {
        self.cypher.visit_expression()
    }
}

impl ExpressionNodeVisitor for ExprGenerator<'_> {
    type Output = String;

    /// expression: or_expression
    fn visit_expression(&mut self) -> Self::Output {
        self.visit_or_expression()
    }

    /// or_expression: xor_expression (OR xor_expression)*
    fn visit_or_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_xor_expression();

        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            ret += " OR ";
            ret += &self.visit_xor_expression();
        }

        // let nmber = self.random.d2();
        // for _ in 0..number {

        // }
        ret
    }

    /// xor_expression: and_expression (XOR and_expression)*
    fn visit_xor_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_and_expression();
        // let number = self.random.d2();
        // for _ in 0..number {
        //     ret += " XOR ";
        //     ret += &self.visit_and_expression();
        // }
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            ret += " XOR ";
            ret += &self.visit_and_expression();
        }
        ret
    }

    fn visit_and_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_not_expression();
        // let number = self.random.d2();
        // // set limit
        // for _ in 0..number {
        //     ret += " AND ";
        //     ret += &self.visit_not_expression();
        // }
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            ret += " AND ";
            ret += &self.visit_not_expression();
        }
        ret
    }

    fn visit_not_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            ret += "NOT ";
        }
        ret += &self.visit_comparison_expression();
        ret
    }

    fn visit_comparison_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        // let number = self.random.d2();
        ret += &self.visit_add_or_subtract_expression();
        // to limit expression complexity.
        // for _ in 0..number {
        //     ret += &self.visit_partial_comparison_expression();
        // }
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            ret += &self.visit_partial_comparison_expression();
        }
        ret
    }

    /// partial_comparison_expression: "=|>|<|<>|<=|>=" add_or_subtract_expression
    fn visit_partial_comparison_expression(&mut self) -> Self::Output {
        // add complexity to limit expression height.
        self.complexity += 1;

        let mut ret = String::new();
        let number = self.random.d6();
        let partial_operators = vec![" = ", " <> ", " > ", " < ", " <= ", " >= "];
        ret += partial_operators[number as usize];
        ret += &self.visit_add_or_subtract_expression();
        ret
    }

    /// add_or_subtract_expression: multiply_divide_modulo_expression (+|- multiply_divide_modulo_expression)*
    fn visit_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_multiply_divide_modulo_expression();
        let add_sub_oper = vec![" + ", " - "];
        // for _ in 0..self.random.d2() {
        //     let opt_number = self.random.d2();
        //     ret += add_sub_oper[opt_number as usize];
        //     ret += &self.visit_multiply_divide_modulo_expression();
        // }
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            self.complexity += 1;

            let opt_number = self.random.d2();
            ret += add_sub_oper[opt_number as usize];
            ret += &self.visit_multiply_divide_modulo_expression();
        }
        ret
    }

    /// multiply_divide_modulo_expression: power_of expression (*|/|% power_of_expression)*
    fn visit_multiply_divide_modulo_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_power_of_expression();
        let add_sub_oper = vec![" * ", " / ", " % "];
        // let loop_number = self.random.d2();
        // for _ in 0..loop_number {
        //     let opt_number = self.random.d6();
        //     if opt_number < 3 {
        //         ret += add_sub_oper[opt_number as usize];
        //         ret += &self.visit_power_of_expression();
        //     }
        // }

        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            let opt_number = self.random.d6();
            if opt_number < 3 {
                self.complexity += 1;
                ret += add_sub_oper[opt_number as usize];
                ret += &self.visit_power_of_expression();
            }
        }

        ret
    }

    /// power_of_expression: unary_add_or_sub_expression (^ unary_add_or_sub_expression)*
    fn visit_power_of_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_unary_add_or_subtract_expression();
        // let loop_number = self.random.d2();
        // for _ in 0..loop_number {
        //     ret += "^";
        //     ret += &self.visit_unary_add_or_subtract_expression();
        // }
        if (self.random.d6() == 1) && (self.complexity < self.limit) {
            self.complexity += 1;

            ret += "^";
            ret += &self.visit_unary_add_or_subtract_expression();
        }
        ret
    }

    // unary
    fn visit_unary_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.random.d20() < 2 {
            self.complexity += 1;
            ret += "+";
        } else if self.random.d20() < 2 {
            self.complexity += 1;
            ret += "-";
        }
        ret += &self.visit_string_list_null_operator_expression();
        ret
    }

    /// this expression:  property_or_labels expression (string_operator|listoperator|nulloperator expression)*
    fn visit_string_list_null_operator_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_property_or_labels_expression();
        // let loop_number = self.random.d2();
        // for _ in 0..loop_number {
        //     match self.random.d12() {
        //         0 => {
        //             ret += &self.visit_string_operator_expression();
        //         }
        //         1 => {
        //             ret += &self.visit_list_operator_expression();
        //         }
        //         2 => {
        //             ret += &self.visit_null_operator_expression();
        //         }
        //         _ => {}
        //     }
        // }

        if (self.random.d2() == 1) && (self.complexity < self.limit) {
            match self.random.d12() {
                0 => {
                    self.complexity += 1;
                    ret += &self.visit_string_operator_expression();
                }
                1 => {
                    self.complexity += 1;
                    ret += &self.visit_list_operator_expression();
                }
                2 => {
                    self.complexity += 1;
                    ret += &self.visit_null_operator_expression();
                }
                _ => {}
            }
        }

        ret
    }

    // property_or_labels_expression: atom property_look_up* nodelabels?(nodelabels*)
    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        // atom
        let mut ret = self.visit_atom();
        // // property*: "."
        // for _ in 0..self.random.d2() {
        //     ret += ".";
        //     ret += "SchemaName(WIP)";
        // }

        // // NodeLabels:
        // for _ in 0..self.random.d2() {
        //     let node_label = NodeLabel::new();
        //     ret += &node_label.get_name();
        // }

        // PropertyKeyName
        if self.random.d6() == 1 {
            ret += ".";
            ret += "SchemaName(WIP)";
        } else if self.random.d6() == 1 {
            // NodelabelName
            let node_label = self.cypher.graph_schema.rand_vertex_label(&mut self.random);
            // let node_label = NodeLabel::new();
            ret += &node_label.get_name();
        }

        ret
    }

    fn visit_string_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.complexity < self.limit {
            match self.random.d9() {
                0 => {
                    self.complexity += 1;
                    ret += " STARTS WITH";
                }
                1 => {
                    self.complexity += 1;
                    ret += " ENDS WITH";
                }
                2 => {
                    self.complexity += 1;
                    ret += " CONTAINS";
                }
                _ => {}
            }
        }
        ret += " ";
        ret += &self.visit_property_or_labels_expression();
        ret
    }

    fn visit_list_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.complexity < self.limit {
            match self.random.d9() {
                0 => {
                    ret += " IN ";
                    ret += &self.visit_property_or_labels_expression();
                }
                1 => {
                    ret += " [";
                    ret += &self.visit_expression();
                    ret += "]";
                }
                2 => {
                    ret += " [";
                    let loop_number = self.random.d100();
                    ret += &self.visit_expression();
                    if loop_number < 2 {
                        for _ in 0..loop_number {
                            ret += " .. ";
                            ret += &self.visit_expression();
                        }
                    }
                    ret += "]";
                }
                _ => {}
            }
        }

        ret += &self.visit_property_or_labels_expression();
        ret
    }

    fn visit_null_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.complexity < self.limit {
            match self.random.d9() {
                0 => {
                    self.complexity += 1;
                    ret += " IS NULL";
                }
                1 => {
                    self.complexity += 1;
                    ret += " IS NOT NULL";
                }
                _ => {}
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::ExprGenerator;
    use crate::ast::CypherGenerator;
    use crate::common::RandomGenerator;

    #[test]
    fn test_expression() {
        let mut cypher_generator = CypherGenerator::new();
        let mut x = ExprGenerator {
            random: RandomGenerator::new(),
            cypher: &mut cypher_generator,
            complexity: 0,
            limit: 10usize,
        };
        let (ans, _) = x.visit();
        println!("{}", ans);
    }

    #[test]
    fn test_atom() {
        let mut cypher_generator = CypherGenerator::new();
        let mut x = ExprGenerator {
            random: RandomGenerator::new(),
            cypher: &mut cypher_generator,
            complexity: 0,
            limit: 10usize,
        };
        let ans = x.visit_atom();
        println!("{}", ans);
    }
}
