mod cypher;
mod cypher_gen;
mod expr;
mod expr_gen;
mod transform;

// pub use expr::{Variable};
pub use cypher::{CypherNode, LogVisitor};
pub use cypher_gen::CypherGenerator;
pub use expr::ExpressionNodeVisitor;
pub use expr_gen::ExprGenerator;
pub use transform::TransformVisitor;

mod constants {
    pub const DEFAULT_EXPRESSION_LIMIT: i32 = 5;
    pub const DEFAULT_QUERY_LIMIT: i32 = 15;
}

#[cfg(test)]
mod tests {

    use super::{constants, CypherGenerator, ExprGenerator};
    use crate::common::{DataType, Property};
    use crate::meta::{GraphSchema, Label, LabelKind};

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
        let ans = x.visit();
        println!("{}", ans.get_name());
    }

    #[test]
    fn expression_with_label_test() {
        let mut labels = vec![];
        // node label: Person {id: i64, name: String}
        let vertex_properties = vec![
            Property {
                name: "id".to_string(),
                prop_id: 0,
                prop_type: DataType::Int32,
                is_pk: true,
                nullable: false,
                is_delete: false,
            },
            Property {
                name: "name".to_string(),
                prop_id: 1,
                prop_type: DataType::String,
                is_pk: false,
                nullable: true,
                is_delete: false,
            },
        ];

        labels.push(Label {
            label_name: "Person".to_string(),
            label_id: 1,
            kind: LabelKind::Vertex,
            properties: vertex_properties.clone(),
        });
        labels.push(Label {
            label_name: "Company".to_string(),
            label_id: 2,
            kind: LabelKind::Vertex,
            properties: vertex_properties,
        });

        let edges_properties = vec![Property {
            name: "edge_id".to_string(),
            prop_id: 0,
            prop_type: DataType::Int32,
            is_pk: true,
            nullable: false,
            is_delete: false,
        }];
        labels.push(Label {
            label_name: "Knows".to_string(),
            label_id: 3,
            kind: LabelKind::Edge {
                relations: vec![(1, 1)],
                is_directed: true,
            },
            properties: edges_properties,
        });
        let graph_schema = GraphSchema::new("test".to_string(), labels);
        let mut generator = CypherGenerator::new_schema(&graph_schema);
        generator.limit = constants::DEFAULT_EXPRESSION_LIMIT;
        let mut expr_generator = ExprGenerator::new(&mut generator);
        let ans = expr_generator.visit();
        println!("{}", ans.get_name());
    }

    #[test]
    fn property_or_labels_expression_test() {
        let mut labels = vec![];
        // node label: Person {id: i64, name: String}
        let vertex_properties = vec![
            Property {
                name: "id".to_string(),
                prop_id: 0,
                prop_type: DataType::Int32,
                is_pk: true,
                nullable: false,
                is_delete: false,
            },
            Property {
                name: "name".to_string(),
                prop_id: 1,
                prop_type: DataType::String,
                is_pk: false,
                nullable: true,
                is_delete: false,
            },
        ];

        labels.push(Label {
            label_name: "Person".to_string(),
            label_id: 1,
            kind: LabelKind::Vertex,
            properties: vertex_properties.clone(),
        });
        labels.push(Label {
            label_name: "Company".to_string(),
            label_id: 2,
            kind: LabelKind::Vertex,
            properties: vertex_properties,
        });

        let edges_properties = vec![Property {
            name: "edge_id".to_string(),
            prop_id: 0,
            prop_type: DataType::Int32,
            is_pk: true,
            nullable: false,
            is_delete: false,
        }];
        labels.push(Label {
            label_name: "Knows".to_string(),
            label_id: 3,
            kind: LabelKind::Edge {
                relations: vec![(1, 1)],
                is_directed: true,
            },
            properties: edges_properties,
        });
        let graph_schema = GraphSchema::new("test".to_string(), labels);
        let mut generator = CypherGenerator::new_schema(&graph_schema);
        generator.limit = constants::DEFAULT_EXPRESSION_LIMIT;
        let expression_string = generator.visit_expression();
        println!("{}", expression_string);
    }

    #[test]
    fn match_clause_test() {
        let mut labels = vec![];
        // node label: Person {id: i64, name: String}
        let vertex_properties = vec![
            Property {
                name: "id".to_string(),
                prop_id: 0,
                prop_type: DataType::Int32,
                is_pk: true,
                nullable: false,
                is_delete: false,
            },
            Property {
                name: "name".to_string(),
                prop_id: 1,
                prop_type: DataType::String,
                is_pk: false,
                nullable: true,
                is_delete: false,
            },
        ];

        labels.push(Label {
            label_name: "Person".to_string(),
            label_id: 1,
            kind: LabelKind::Vertex,
            properties: vertex_properties.clone(),
        });
        labels.push(Label {
            label_name: "Company".to_string(),
            label_id: 2,
            kind: LabelKind::Vertex,
            properties: vertex_properties,
        });

        let edges_properties = vec![Property {
            name: "edge_id".to_string(),
            prop_id: 0,
            prop_type: DataType::Int32,
            is_pk: true,
            nullable: false,
            is_delete: false,
        }];
        labels.push(Label {
            label_name: "Knows".to_string(),
            label_id: 3,
            kind: LabelKind::Edge {
                relations: vec![(1, 1)],
                is_directed: true,
            },
            properties: edges_properties,
        });
        let graph_schema = GraphSchema::new("test".to_string(), labels);
        let mut generator = CypherGenerator::new_schema(&graph_schema);
        let (_, expression_string) = generator.test_match_clause();
        println!("{}", expression_string);
    }
}
