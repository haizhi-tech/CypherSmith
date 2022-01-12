use super::RandomGenerator;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub type LabelId = u16;
pub type PropertyId = u16;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    Null = 0,
    Bool = 1,
    Int32 = 2,
    Int64 = 3,
    Float = 4,
    Double = 5,
    Date = 6,
    Datetime = 7,
    String = 8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FieldValue {
    Null,
    Boolean(bool),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Date(i64),
    Datetime(i64),
    String(String),
}

impl FieldValue {
    /// Random Generate Default Value.
    pub fn get_default_value(d_type: DataType) -> FieldValue {
        let mut random = RandomGenerator::new();
        match d_type {
            DataType::Null => Self::Null,
            DataType::Bool => Self::Boolean(random.bool()),
            DataType::Int32 => Self::Int32(random.d6()),
            DataType::Int64 => Self::Int64(random.d6() as _),
            DataType::Float => Self::Float(random.d6() as _),
            DataType::Double => Self::Double(random.d6() as _),
            DataType::Date => Self::Date(0),
            DataType::Datetime => Self::Datetime(0),
            DataType::String => Self::String(random.d6().to_string()),
        }
    }
}

impl Default for FieldValue {
    fn default() -> Self {
        FieldValue::Null
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match self {
                Self::Null => "Null".to_string(),
                Self::Boolean(b) => {
                    if *b {
                        "True".to_string()
                    } else {
                        "False".to_string()
                    }
                }
                Self::Int32(i) => i.to_string(),
                Self::Int64(i) => i.to_string(),
                Self::Float(f) => f.to_string(),
                Self::Double(d) => d.to_string(),
                Self::Date(d) => d.to_string(), // should add duration from 1970 and to string
                Self::Datetime(d) => d.to_string(),
                Self::String(s) => "'".to_string() + s + "'",
            }
            .as_ref(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub prop_id: PropertyId,
    pub prop_type: DataType,
    pub is_pk: bool,
    pub nullable: bool,
    pub is_delete: bool,
}

impl Property {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn default_value(&self) -> FieldValue {
        FieldValue::get_default_value(self.prop_type)
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{DataType, Property};

    #[test]
    fn test_property() {
        let pro = Property {
            name: "age".to_string(),
            prop_id: 1,
            prop_type: DataType::Int32,
            is_pk: false,
            nullable: true,
            is_delete: false,
        };

        println!("{}", pro)
    }
}
