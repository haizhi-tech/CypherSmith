pub type LabelId = u16;
pub type PropertyId = u16;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    prop_id: PropertyId,
    prop_type: DataType,
    is_pk: bool,
    nullable: bool,
    is_delete: bool,
    is_vertex: bool,
    is_index: bool,
    is_unique: bool,
}