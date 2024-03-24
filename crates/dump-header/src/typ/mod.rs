use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use clang::TypeKind;
use serde::{Deserialize, Serialize};

mod typekind;
use typekind::TypeKindDef;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Nullability {
    NonNull = 0,
    Nullable = 1,
    Unspecified = 2,
}

impl Nullability {
    pub fn from(nullability: clang::Nullability) -> Self {
        match nullability {
            clang::Nullability::NonNull => Self::NonNull,
            clang::Nullability::Nullable => Self::Nullable,
            clang::Nullability::Unspecified => Self::Unspecified,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordField {
    name: Option<String>,
    is_anonymous: Option<bool>,
    ty: Typ,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind")]
pub enum Typ {
    Pointer {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        pointee_type: Box<Typ>,
        is_const: bool,
    },
    FunctionPrototype {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
        argument_types: Option<Vec<Typ>>,
        result_type: Option<Box<Typ>>,
    },
    CArray {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        element_type: Box<Typ>,
        size: Option<usize>,
        is_const: bool,
    },
    StructRecord {
        name: Option<String>,
        ident: Option<String>,
        fields: Vec<RecordField>,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
    },
    UnionRecord {
        name: Option<String>,
        ident: Option<String>,
        fields: Vec<RecordField>,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
    },
    // prevents infinite recursive loop for recursive struct
    RecordIdent {
        ident: String,
    },
    ObjC {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        objc_type_arguments: Vec<Typ>,
        is_const: bool,
    },
    OtherType {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
    },
}

impl Typ {
    pub fn from(ty: clang::Type) -> Self {
        Self::from0(ty, Rc::new(RefCell::new(HashSet::new())))
    }

    fn from0(ty: clang::Type, memo: Rc<RefCell<HashSet<String>>>) -> Self {
        let name = ty.get_display_name();
        let nullability = ty.get_nullability().map(Nullability::from);
        match ty.get_kind() {
            TypeKind::Attributed | TypeKind::Elaborated => {
                let canonical_ty = ty.get_canonical_type();
                Self::from_impl(canonical_ty, name, nullability, memo)
            }
            _ => Self::from_impl(ty, name, nullability, memo),
        }
    }

    fn from_impl(
        ty: clang::Type,
        name: String,
        nullability: Option<Nullability>,
        memo: Rc<RefCell<HashSet<String>>>,
    ) -> Self {
        let clang_kind = ty.get_kind();
        let objc_encoding = if clang_kind == clang::TypeKind::ObjCObject {
            Some("@".to_string())
        } else {
            // [WORKAROUND] can cause segfault!!
            ty.get_objc_encoding()
        };
        let is_const = ty.is_const_qualified();

        match clang_kind {
            TypeKind::Pointer
            | TypeKind::ObjCObjectPointer
            | TypeKind::BlockPointer
            | TypeKind::MemberPointer => Self::Pointer {
                name,
                clang_kind,
                nullability,
                objc_encoding,
                pointee_type: ty
                    .get_pointee_type()
                    .map(|t| Box::new(Typ::from0(t, memo)))
                    .unwrap(),
                is_const,
            },
            TypeKind::FunctionNoPrototype | TypeKind::FunctionPrototype => {
                Self::FunctionPrototype {
                    name,
                    clang_kind,
                    nullability,
                    objc_encoding,
                    is_const,
                    argument_types: ty
                        .get_argument_types()
                        .map(|t| t.iter().map(|t| Typ::from0(*t, memo.clone())).collect()),
                    result_type: ty.get_result_type().map(|t| Box::new(Typ::from0(t, memo))),
                }
            }
            TypeKind::ConstantArray
            | TypeKind::IncompleteArray
            | TypeKind::DependentSizedArray
            | TypeKind::VariableArray => Self::CArray {
                name,
                clang_kind,
                nullability,
                objc_encoding,
                element_type: Box::new(Typ::from0(ty.get_element_type().unwrap(), memo)),
                size: ty.get_size(),
                is_const,
            },
            TypeKind::Record => {
                let entity = ty.get_declaration().unwrap();
                let is_anonymous = entity.is_anonymous_record_decl();
                let ident = if !is_anonymous {
                    entity.get_name()
                } else {
                    None
                };
                let name = if !is_anonymous { Some(name) } else { None };
                if let Some(ref ident) = ident {
                    if memo.borrow().contains(ident) {
                        return Self::RecordIdent {
                            ident: ident.clone(),
                        };
                    }
                    memo.borrow_mut().insert(ident.clone());
                }
                let fields: Vec<RecordField> = ty
                    .get_fields()
                    .unwrap()
                    .iter()
                    .map(|e| RecordField {
                        name: e.get_name(),
                        is_anonymous: e.get_type().and_then(|t| {
                            t.get_declaration().map(|e| e.is_anonymous_record_decl())
                        }),
                        ty: Typ::from0(e.get_type().unwrap(), memo.clone()),
                    })
                    .collect();
                match entity.get_kind() {
                    clang::EntityKind::StructDecl => Self::StructRecord {
                        name,
                        ident,
                        fields,
                        clang_kind,
                        nullability,
                        objc_encoding,
                        is_const,
                    },
                    clang::EntityKind::UnionDecl => Self::UnionRecord {
                        name,
                        ident,
                        fields,
                        clang_kind,
                        nullability,
                        objc_encoding,
                        is_const,
                    },
                    _ => panic!("unexpected entity kind: {:?}", entity.get_kind()),
                }
            }
            TypeKind::ObjCClass
            | TypeKind::ObjCId
            | TypeKind::ObjCInterface
            | TypeKind::ObjCObject
            | TypeKind::ObjCSel
            | TypeKind::ObjCTypeParam => Self::ObjC {
                name,
                clang_kind,
                nullability,
                objc_encoding,
                objc_type_arguments: ty
                    .get_objc_type_arguments()
                    .iter()
                    .map(|t| Typ::from0(*t, memo.clone()))
                    .collect(),
                is_const,
            },
            _ => Self::OtherType {
                name,
                clang_kind,
                nullability,
                objc_encoding,
                is_const,
            },
        }
    }
}
