use clang::Entity;

pub enum InitExpr<'tu> {
    Value(InitValue),
    Entity(Entity<'tu>),
}

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

pub fn get_init_value(entity: clang::Entity) -> Option<InitExpr> {
    if let Some(value) = entity.evaluate() {
        match value {
            clang::EvaluationResult::SignedInteger(value) => {
                Some(InitExpr::Value(InitValue::Int(value)))
            }
            clang::EvaluationResult::UnsignedInteger(value) => {
                Some(InitExpr::Value(InitValue::UInt(value)))
            }
            clang::EvaluationResult::Float(value) => Some(InitExpr::Value(InitValue::Float(value))),
            clang::EvaluationResult::String(value)
            | clang::EvaluationResult::ObjCString(value)
            | clang::EvaluationResult::CFString(value)
            | clang::EvaluationResult::Other(value) => Some(InitExpr::Value(InitValue::String(
                value.to_string_lossy().to_string(),
            ))),
            _ => None,
        }
    } else {
        entity.get_children().iter().find_map(|e| {
            if clang::EntityKind::InitListExpr == e.get_kind() {
                Some(InitExpr::Entity(entity))
            } else {
                None
            }
        })
    }
}
