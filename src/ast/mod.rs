mod cypher;
mod cypher_gen;
mod expr;
mod expr_gen;
mod prod;
mod transform;

// pub use expr::{Variable};
pub use cypher::CypherNode;
pub use cypher_gen::CypherGenerator;
pub use expr::ExpressionNodeVisitor;
pub use transform::TransformVisitor;

#[cfg(test)]
mod tests {

    use super::cypher_gen::CypherGenerator;

    #[test]
    fn query_test() {
        let mut generator = CypherGenerator::new();
        generator.visit();
        println!("{}", generator.get_current_query_string());
    }

    #[test]
    fn property_or_labels_expression_test() {
        let mut generator = CypherGenerator::new();
        let expression_string = generator.visit_expression();
        println!("{}", expression_string);
    }
}
