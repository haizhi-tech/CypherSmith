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
pub use expr_gen::ExprGenerator;
pub use transform::TransformVisitor;

#[cfg(test)]
mod tests {

    use super::{CypherGenerator, ExprGenerator};

    #[test]
    fn cypher_generator_test() {
        let mut generator = CypherGenerator::new();
        let (_, cypher_string) = generator.visit();
        println!("{}", cypher_string);
    }

    #[test]
    fn expression_generator_test() {
        let mut cypher_generator = CypherGenerator::new();
        let mut x = ExprGenerator::new(&mut cypher_generator);
        let (ans, _) = x.visit();
        println!("{}", ans);
    }

    #[test]
    fn property_or_labels_expression_test() {
        let mut generator = CypherGenerator::new();
        let expression_string = generator.visit_expression();
        println!("{}", expression_string);
    }
}
