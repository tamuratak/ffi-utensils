use clang::Entity;

pub enum InitValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<InitValue>),
    Struct(Vec<(String, InitValue)>),
    Null,
}

pub fn get_init_value(entity: clang::Entity) -> Option<InitValue> {
    None
}
