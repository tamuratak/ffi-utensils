use std::collections::HashSet;
use std::cell::RefCell;

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
    Record {
        name: String,
        ident: String,
        fields: Vec<(String, Typ)>,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
    },
    RecordIdent {
        ident: String
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
        Self::from0(ty, HashSet::new())
    }

    fn from0(ty: clang::Type, memo: HashSet<String>) -> Self {
        let name = ty.get_display_name();
        let nullability = ty.get_nullability().map(|n| Nullability::from(n));
        match ty.get_kind() {
            TypeKind::Attributed | TypeKind::Elaborated => {
                let canonical_ty = ty.get_canonical_type();
                Self::from_impl(canonical_ty, name, nullability, memo)
            }
            _ => Self::from_impl(ty, name, nullability, memo),
        }
    }

    fn from_impl(ty: clang::Type, name: String, nullability: Option<Nullability>, mut memo: HashSet<String>) -> Self {
        let clang_kind = ty.get_kind();
        let objc_encoding = ty.get_objc_encoding();
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
                    .map(|t| Box::new(Typ::from0(t, memo.clone())))
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
                    result_type: ty.get_result_type().map(|t| Box::new(Typ::from0(t, memo.clone()))),
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
                element_type: Box::new(Typ::from0(ty.get_element_type().unwrap(), memo.clone())),
                size: ty.get_size(),
                is_const,
            },
            TypeKind::Record => {
                let ident = ty.get_declaration().unwrap().get_name().unwrap();
                if memo.contains(&ident) {
                    return Self::RecordIdent {
                        ident
                    };
                } else {
                    memo.insert(ident.clone());
                }
                Self::Record {
                    name,
                    ident,
                    fields: ty
                        .get_fields()
                        .unwrap()
                        .iter()
                        .map(|e| {
                            (e.get_name().unwrap(), Typ::from0(e.get_type().unwrap(), memo.clone()))
                        })
                        .collect(),
                    clang_kind,
                    nullability,
                    objc_encoding,
                    is_const,
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
