use clang::TypeKind;
use serde::{Deserialize, Serialize};

mod typekind;
use typekind::TypeKindDef;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Type {
    Pointer {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        pointee_type: Box<Type>,
        is_const: bool,
    },
    FunctionPrototype {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        is_const: bool,
        argument_types: Option<Vec<Type>>,
        result_type: Option<Box<Type>>,
    },
    CArray {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        element_type: Box<Type>,
        size: Option<usize>,
        is_const: bool,
    },
    ObjC {
        name: String,
        #[serde(with = "TypeKindDef")]
        clang_kind: clang::TypeKind,
        nullability: Option<Nullability>,
        objc_encoding: Option<String>,
        objc_type_arguments: Vec<Type>,
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

impl Type {
    pub fn from(ty: clang::Type) -> Self {
        let name = ty.get_display_name();
        let nullability = ty.get_nullability().map(|n| Nullability::from(n));
        if ty.get_kind() == TypeKind::Attributed {
            Self::from_impl(ty.get_canonical_type(), name, nullability)
        } else {
            Self::from_impl(ty, name, nullability)
        }
    }

    fn from_impl(ty: clang::Type, name: String, nullability: Option<Nullability>) -> Self {
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
                    .map(|t| Box::new(Type::from(t)))
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
                        .map(|t| t.iter().map(|t| Type::from(*t)).collect()),
                    result_type: ty.get_result_type().map(|t| Box::new(Type::from(t))),
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
                element_type: Box::new(Type::from(ty.get_element_type().unwrap())),
                size: ty.get_size(),
                is_const,
            },
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
                    .map(|t| Type::from(*t))
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
