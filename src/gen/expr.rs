use crate::ast::{ExpressionNodeVisitor, CypherGenerator};

use crate::common::RandomGenerator;

pub struct ExprGenVisitor<'a> {
    expr_string: String,
    random: RandomGenerator,
    cypher: &'a mut CypherGenerator,
}

// impl ExprGenVisitor<'_> {
//     pub fn new (cypher: &mut CypherGenerator) -> Self {
//         ExprGenVisitor {
//             random: RandomGenerator::new(),
//             expr_string: String::new(),
//             cypher,
//         }
//     }
// }

impl ExprGenVisitor<'_> {
    pub fn visit(&mut self) -> String {
        self.expr_string = self.visit_expression();
        self.expr_string.clone()
    }

    pub fn get_name(&self) -> String {
        self.expr_string.clone()
    }

    pub fn visit_atom(&mut self) -> String {
        let x = self.cypher.visit_atom_expression();
        x
    }
}

impl ExpressionNodeVisitor for ExprGenVisitor<'_> {
    type Output = String;

    /// expression: or_expression
    fn visit_expression(&mut self) -> Self::Output {
        self.visit_or_expression()
    }

    /// or_expression: xor_expression (OR xor_expression)*
    fn visit_or_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_xor_expression();
        let number = self.random.d2();
        for _ in 0..number {
            ret += " OR ";
            ret += &self.visit_xor_expression();
        }
        ret
    }

    /// xor_expression: and_expression (XOR and_expression)*
    fn visit_xor_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_and_expression();
        let number = self.random.d2();
        for _ in 0..number {
            ret += " XOR ";
            ret += &self.visit_and_expression();
        }
        ret
    }

    fn visit_and_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_not_expression();
        let number = self.random.d2();
        // set limit
        for _ in 0..number {
            ret += " AND ";
            ret += &self.visit_not_expression();
        }
        ret
    }

    fn visit_not_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.random.bool() {
            ret += "NOT ";
        }
        ret += &self.visit_comparison_expression();
        ret
    }

    fn visit_comparison_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        let number = self.random.d2();
        ret += &self.visit_add_or_subtract_expression();
        // to limit expression complexity.
        for _ in 0..number {
            ret += &self.visit_partial_comparison_expression();
        }
        ret
    }

    /// partial_comparison_expression: "=|>|<|<>|<=|>=" add_or_subtract_expression
    fn visit_partial_comparison_expression(&mut self) -> Self::Output {
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
        for _ in 0..self.random.d2() {
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
        let loop_number = self.random.d2();
        for _ in 0..loop_number {
            let opt_number = self.random.d6();
            if opt_number < 3 {
                ret += add_sub_oper[opt_number as usize];
                ret += &self.visit_power_of_expression();
            }
        }
        ret
    }

    /// power_of_expression: unary_add_or_sub_expression (^ unary_add_or_sub_expression)*
    fn visit_power_of_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_unary_add_or_subtract_expression();
        let loop_number = self.random.d2();
        for _ in 0..loop_number {
            ret += "^";
            ret += &self.visit_unary_add_or_subtract_expression();
        }
        ret
    }

    // unary
    fn visit_unary_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        if self.random.d20() < 2 {
            ret += "+";
        } else if self.random.d20() < 2 {
            ret += "-";
        }
        ret += &self.visit_string_list_null_operator_expression();
        ret
    }

    /// this expression:  property_or_labels expression (string_operator|listoperator|nulloperator expression)*
    fn visit_string_list_null_operator_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_property_or_labels_expression();
        let loop_number = self.random.d2();
        for _ in 0..loop_number {
            match self.random.d12() {
                0 => {
                    ret += &self.visit_string_operator_expression();
                }
                1 => {
                    ret += &self.visit_list_operator_expression();
                }
                2 => {
                    ret += &self.visit_null_operator_expression();
                }
                _ => {}
            }
        }

        ret
    }

    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        "property(WIP)".to_string()
    }

    fn visit_string_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        match self.random.d6() {
            0 => {
                ret += " STARTS WITH";
            }
            1 => {
                ret += " ENDS WITH";
            }
            2 => {
                ret += " CONTAINS";
            }
            _ => {}
        }
        ret += " ";
        ret += &self.visit_property_or_labels_expression();
        ret
    }

    fn visit_list_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        match self.random.d6() {
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
        ret += &self.visit_property_or_labels_expression();
        ret
    }

    fn visit_null_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        match self.random.d6() {
            0 => {
                ret += " IS NULL";
            }
            1 => {
                ret += " IS NOT NULL";
            }
            _ => {}
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::ExprGenVisitor;
    use crate::common::RandomGenerator;
    use crate::ast::CypherGenerator;

    #[test]
    fn test_new() {
        let mut cypher_generator = CypherGenerator::new();
        let mut x = ExprGenVisitor {
            random: RandomGenerator::new(),
            expr_string: String::new(),
            cypher: &mut cypher_generator,
        };
        let ans = x.visit();
        println!("{}", ans);
    }
}
