use crate::ast::ExpressionNodeVisitor;

use crate::common::RandomGenerator;

pub struct ExprGenVisitor {
    expr_string: String,
    random: RandomGenerator,
    height: u32,
    limit: u32,
}

impl ExprGenVisitor {
    pub fn new() -> Self {
        ExprGenVisitor {
            random: RandomGenerator::new(),
            expr_string: String::new(),
            height: 0u32,
            limit: 100u32,
        }
    }
}

impl ExprGenVisitor {
    pub fn visit(&mut self) -> String {
        self.expr_string = self.visit_expression();
        self.expr_string.clone()
    }

    pub fn get_name(&self) -> String {
        self.expr_string.clone()
    }
}

impl ExpressionNodeVisitor for ExprGenVisitor {
    type Output = String;

    /// expression: or_expression
    fn visit_expression(&mut self) -> Self::Output {
        self.visit_or_expression()
    }

    /// or_expression: xor_expression (OR xor_expression)*
    fn visit_or_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_xor_expression();
        let number = self.random.d100();
        if number < 4 {
            for _ in 0..number {
                ret += " OR ";
                ret += &self.visit_xor_expression();
            }
        }
        ret
    }

    /// xor_expression: and_expression (XOR and_expression)*
    fn visit_xor_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_and_expression();
        let number = self.random.d100();
        if number < 4 {
            for _ in 0..number {
                ret += " XOR ";
                ret += &self.visit_and_expression();
            }
        }
        ret
    }

    fn visit_and_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        ret += &self.visit_not_expression();
        let number = self.random.d100();
        // set limit
        if number < 4 {
            for _ in 0..number {
                ret += " AND ";
                ret += &self.visit_not_expression();
            }
        }
        ret
    }

    fn visit_not_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        let number = self.random.d100();
        // 1/2 prob.
        if number < 4 {
            ret += " NOT ";
        }
        ret += &self.visit_comparison_expression();
        ret
    }

    fn visit_comparison_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        let number = self.random.d100();
        ret += &self.visit_add_or_subtract_expression();
        // to limit expression complexity.
        if number < 4 {
            for _ in 0..number {
                ret += " \n";
                ret += &self.visit_partial_comparison_expression();
            }
        }
        ret
    }

    /// partial_comparison_expression: "=|>|<|<>|<=|>=" add_or_subtract_expression
    fn visit_partial_comparison_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        let number = self.random.d100();
        if number < 6 {
            let partial_operators = vec![" = ", " <> ", " > ", " < ", " <= ", " >= "];
            ret += partial_operators[number as usize];
            ret += &self.visit_add_or_subtract_expression();
        }
        ret
    }

    /// add_or_subtract_expression: multiply_divide_modulo_expression (+|- multiply_divide_modulo_expression)*
    fn visit_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_multiply_divide_modulo_expression();
        let loop_number = self.random.d6();
        for _ in 0..loop_number {
            let opt_number = self.random.d100();
            if opt_number < 2 {
                let add_sub_oper = vec![" + ", " - "];
                ret += add_sub_oper[opt_number as usize];
                ret += &self.visit_multiply_divide_modulo_expression();
            }
        }
        ret
    }

    /// multiply_divide_modulo_expression: power_of expression (*|/|% power_of_expression)*
    fn visit_multiply_divide_modulo_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_power_of_expression();
        let loop_number = self.random.d6();
        for _ in 0..loop_number {
            let opt_number = self.random.d100();
            if opt_number < 3 {
                let add_sub_oper = vec![" * ", " / ", " % "];
                ret += add_sub_oper[opt_number as usize];
                ret += &self.visit_power_of_expression();
            }
        }
        ret
    }

    /// power_of_expression: unary_add_or_sub_expression (^ unary_add_or_sub_expression)*
    fn visit_power_of_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_unary_add_or_subtract_expression();
        let loop_number = self.random.d100();
        if loop_number <= 2 {
            for _ in 0..loop_number {
                ret += "^";
                ret += &self.visit_unary_add_or_subtract_expression();
            }
        }
        ret
    }

    // un
    fn visit_unary_add_or_subtract_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        let number = self.random.d100();
        // 1/2 prob.
        if number < 2 {
            ret += "+";
        } else if number < 4 {
            ret += "-";
        }
        ret += &self.visit_string_list_null_operator_expression();
        ret
    }

    /// this expression:  property_or_labels expression (string_operator|listoperator|nulloperator expression)*
    fn visit_string_list_null_operator_expression(&mut self) -> Self::Output {
        let mut ret = self.visit_property_or_labels_expression();
        let loop_number = self.random.d6();
        for _ in 0..loop_number {
            match self.random.d100() {
                0 => {
                    ret += "\n";
                    ret += &self.visit_string_operator_expression();
                }
                1 => {
                    ret += "\n";
                    ret += &self.visit_list_operator_expression();
                }
                2 => {
                    ret += "\n";
                    ret += &self.visit_null_operator_expression();
                }
                _ => {}
            }
        }

        ret
    }

    fn visit_property_or_labels_expression(&mut self) -> Self::Output {
        "1".to_string()
    }

    fn visit_string_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        match self.random.d100() {
            0 => {
                ret += "STARTS WITH ";
            }
            1 => {
                ret += "ENDS WITH ";
            }
            2 => {
                ret += "CONTAINS ";
            }
            _ => {}
        }
        ret += &self.visit_property_or_labels_expression();
        ret
    }

    fn visit_list_operator_expression(&mut self) -> Self::Output {
        let mut ret = String::new();
        match self.random.d100() {
            0 => {
                ret += "IN";
                ret += &self.visit_property_or_labels_expression();
            }
            1 => {
                ret += "[";
                ret += &self.visit_expression();
                ret += "]";
            }
            2 => {
                ret += "[";
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
        match self.random.d100() {
            0 => {
                ret += "IS NULL";
            }
            1 => {
                ret += "IS NOT NULL";
            }
            _ => {}
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::ExprGenVisitor;

    #[test]
    fn test_new() {
        let mut x = ExprGenVisitor::new();
        let ans = x.visit();
        println!("{}", ans);
    }
}
