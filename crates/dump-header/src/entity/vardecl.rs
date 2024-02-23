use super::entry::{InitExpr, InitListExpr, InitValue};

pub fn get_init_expr(entity: &clang::Entity) -> Option<InitExpr> {
    if let Some(value) = evaluate(&entity) {
        Some(InitExpr::Value(value))
    } else {
        entity.get_children().iter().find_map(|e| {
            if clang::EntityKind::InitListExpr == e.get_kind() {
                if let Some(init_list_expr) = get_init_list_expr(e) {
                    return Some(InitExpr::InitListExpr(init_list_expr));
                }
            }
            None
        })
    }
}

fn evaluate(entity: &clang::Entity) -> Option<InitValue> {
    if let Some(value) = entity.evaluate() {
        match value {
            clang::EvaluationResult::SignedInteger(value) => Some(InitValue::Int(value)),
            clang::EvaluationResult::UnsignedInteger(value) => Some(InitValue::UInt(value)),
            clang::EvaluationResult::Float(value) => Some(InitValue::Float(value)),
            clang::EvaluationResult::String(value)
            | clang::EvaluationResult::ObjCString(value)
            | clang::EvaluationResult::CFString(value)
            | clang::EvaluationResult::Other(value) => {
                Some(InitValue::String(value.to_string_lossy().to_string()))
            }
            _ => None,
        }
    } else {
        None
    }
}

fn get_init_list_expr(entity: &clang::Entity) -> Option<InitListExpr> {
    let mut values = vec![];
    entity.get_children().iter().for_each(|e| {
        if let Some(value) = evaluate(e) {
            values.push(value);
        }
    });
    if values.is_empty() {
        None
    } else {
        Some(InitListExpr { values })
    }
}
