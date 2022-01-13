use csv::Writer;
use serde::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Errors {
    cypher: String,
    errors: Vec<String>,
}

pub struct OutputWriter {
    pub file: Writer<std::fs::File>,
}

impl OutputWriter {
    pub fn new(path: String) -> Self {
        let file = csv::Writer::from_path(path).unwrap();
        Self { file }
    }

    pub fn write_errors(&mut self, cypher: String, errors: Vec<String>) {
        // let record = serde_json::from_value::<Errors>(val).unwrap();
        let record = Errors {
            cypher,
            errors,
        };
        // the header row written automatic
        self.file.serialize(record).unwrap();
    }
}

#[allow(dead_code)]
pub const RESERVED_WORD: &[&str] = &[
    "All",
    "And",
    "As",
    "Asc",
    "Ascending",
    "By",
    "Case",
    "Create",
    "Delete",
    "Desc",
    "Descending",
    "Detach",
    "Delete",
    "Distinct",
    "Drop",
    "Else",
    "End",
    "Ends",
    "Exists",
    "False",
    "In",
    "Is",
    "Limit",
    "Match",
    "Merge",
    "Not",
    "Null",
    "On",
    "Optional",
    "Or",
    "Order",
    "Remove",
    "Return",
    "Set",
    "Skip",
    "Starts",
    "Then",
    "To",
    "True",
    "Union",
    "Unique",
    "Unwind",
    "When",
    "Where",
    "With",
    "Xor",
];
