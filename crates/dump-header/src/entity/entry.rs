use std::path::PathBuf;

use super::attributes::ObjCAttributes;
use crate::typ::Typ;
use serde::{Deserialize, Serialize};

use super::availability::{AvailabilityDef, PlatformAvailability};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind")]
pub enum Entry {
    InclusionDirective {
        name: String,
        path: PathBuf,
    },
    TypedefDecl {
        name: String,
        ty: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    EnumDecl {
        decls: Vec<EnumConstantDecl>,
        name: String,
        ty: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    VarDecl {
        name: String,
        ty: Typ,
        init_expr: Option<InitExpr>,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    StructDecl {
        name: Option<String>,
        fields: Vec<Entry>,
        ty: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    FieldDecl {
        name: String,
        ty: Typ,
    },
    UnionDecl {
        name: Option<String>,
        ty: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    FunctionDecl {
        name: String,
        ty: Typ,
        arguments: Vec<ParmDecl>,
        result_type: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    ObjCInterfaceDecl {
        name: String,
        template_args: Vec<TemplateTypeParameter>,
        superclass: String,
        protocols: Vec<String>,
        properties: Vec<ObjCPropertyDecl>,
        instance_methods: Vec<ObjCMethodDecl>,
        class_methods: Vec<ObjCMethodDecl>,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    ObjCCategoryDecl {
        name: Option<String>,
        class_name: String,
        properties: Vec<ObjCPropertyDecl>,
        instance_methods: Vec<ObjCMethodDecl>,
        class_methods: Vec<ObjCMethodDecl>,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    ObjCProtocolDecl {
        name: String,
        properties: Vec<ObjCPropertyDecl>,
        instance_methods: Vec<ObjCMethodDecl>,
        class_methods: Vec<ObjCMethodDecl>,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind")]
pub enum InitExpr {
    Value(InitValue),
    InitListExpr(InitListExpr),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitListExpr {
    pub values: Vec<InitValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjCMethodDecl {
    pub name: String,
    pub arguments: Vec<ParmDecl>,
    pub result_type: Typ,
    pub optional: bool,
    pub platform_availability: Option<Vec<PlatformAvailability>>,
    #[serde(with = "AvailabilityDef")]
    pub availability: clang::Availability,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjCPropertyDecl {
    pub name: String,
    pub objc_type: Typ,
    pub optional: bool,
    pub attributes: Option<ObjCAttributes>,
    pub platform_availability: Option<Vec<PlatformAvailability>>,
    #[serde(with = "AvailabilityDef")]
    pub availability: clang::Availability,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParmDecl {
    pub name: Option<String>,
    pub objc_type: Typ,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateTypeParameter {
    pub name: String,
    pub constraint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumConstantDecl {
    pub name: String,
    pub value: Option<String>,
    pub objc_type: Typ,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDecl {
    pub name: String,
    pub objc_type: Typ,
}
