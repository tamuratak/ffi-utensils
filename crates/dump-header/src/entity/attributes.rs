use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjCAttributes {
    pub readonly: bool,
    pub getter: bool,
    pub assign: bool,
    pub readwrite: bool,
    pub retain: bool,
    pub copy: bool,
    pub nonatomic: bool,
    pub setter: bool,
    pub atomic: bool,
    pub weak: bool,
    pub strong: bool,
    pub unsafe_retained: bool,
}

impl ObjCAttributes {
    pub fn from(attributes: clang::ObjCAttributes) -> Self {
        Self {
            readonly: attributes.readonly,
            getter: attributes.getter,
            assign: attributes.assign,
            readwrite: attributes.readwrite,
            retain: attributes.retain,
            copy: attributes.copy,
            nonatomic: attributes.nonatomic,
            setter: attributes.setter,
            atomic: attributes.atomic,
            weak: attributes.weak,
            strong: attributes.strong,
            unsafe_retained: attributes.unsafe_retained,
        }
    }   
}
