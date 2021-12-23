use common::{meta::field_value::DataType, prelude::ShardId, typedef::ReplicaFactor};
pub use rpc::atlas::PropertyType as RpcPropertyType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Schema {
    // version: u16,
    pub graph_name: String,
    #[serde(default = "default_shard_num")]
    pub shard_num: ShardId,
    #[serde(default = "default_replica_factor")]
    pub replica_factor: ReplicaFactor,
    #[serde(default = "String::new")]
    pub description: String,

    pub vertices: HashMap<String, Vertex>,
    pub edges: HashMap<String, Edge>,
}

fn default_shard_num() -> ShardId {
    common::config::MetaServer::default().shard_num
}

fn default_replica_factor() -> ReplicaFactor {
    common::config::MetaServer::default().replications
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Vertex {
    pub properties: Vec<Property>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Edge {
    // pub src_label_name: String,
    // pub dst_label_name: String,
    pub properties: Vec<Property>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(serialize_with = "crate::data_type_ext::ser_data_type_to_str")]
    #[serde(deserialize_with = "crate::data_type_ext::de_data_type_from_str")]
    type_: DataType,
    #[serde(default = "bool::default")]
    #[serde(skip_serializing_if = "is_false")]
    is_pk: bool,
    #[serde(default = "bool::default")]
    #[serde(skip_serializing_if = "is_false")]
    is_unique: bool,
    #[serde(default = "bool::default")]
    #[serde(skip_serializing_if = "is_false")]
    is_index: bool,
    #[serde(default = "default_true")]
    nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<common::meta::FieldValue>,
}

/// only used in serde
#[inline]
const fn default_true() -> bool {
    true
}

/// only used in serde
#[inline]
const fn is_false(bool_: &bool) -> bool {
    !(*bool_)
}

impl From<RpcPropertyType> for Property {
    fn from(prop: RpcPropertyType) -> Self {
        Self {
            name: prop.name,
            type_: DataType::from_i32(prop.prop_type),
            is_pk: prop.is_pk,
            is_unique: prop.is_unique,
            is_index: prop.is_index,
            nullable: prop.nullable,
            default_value: prop.default_value.map(Into::into),
        }
    }
}

/**
* is_vertex need to set after From/Into

* meta server would ignore prop_id in request
    so prop id can be skip in deserialize from json
*/
impl From<Property> for RpcPropertyType {
    fn from(prop: Property) -> Self {
        debug_assert_ne!(prop.nullable, prop.is_pk);
        Self {
            name: prop.name,
            prop_type: prop.type_.into(),
            is_pk: prop.is_pk,
            is_unique: prop.is_unique,
            is_index: prop.is_index,
            nullable: prop.nullable,
            default_value: prop.default_value.map(Into::into),
            ..Default::default()
        }
    }
}

#[test]
fn test_schema_deserialize() {
    const JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sample/nba/schema.json"));
    let schema = serde_json::from_str::<Schema>(JSON).unwrap();
    let json = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", json);
}

impl Schema {
    /// 1. check whether duplicates in filed/property name
    /// 2. check primary key whether set
    pub fn check_valid(&self) {
        use std::collections::HashSet;
        for vertex in self.vertices.values() {
            assert_eq!(vertex.properties.iter().filter(|prop| prop.is_pk).count(), 1);
            if vertex
                .properties
                .iter()
                .map(|prop| prop.name.clone())
                .collect::<HashSet<_>>()
                .len()
                != vertex.properties.len()
            {
                panic!("duplicate field in schema.json property");
            }
        }
        for edge in self.edges.values() {
            assert_eq!(edge.properties.iter().filter(|prop| prop.is_pk).count(), 0);
            if edge
                .properties
                .iter()
                .map(|prop| prop.name.clone())
                .collect::<HashSet<_>>()
                .len()
                != edge.properties.len()
            {
                panic!("duplicate field in schema.json property");
            }
        }
    }
}
