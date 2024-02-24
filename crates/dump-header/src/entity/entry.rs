use std::path::PathBuf;

use super::attributes::ObjCAttributes;
use crate::typ::Typ;
use serde::{Deserialize, Serialize};

use super::availability::{PlatformAvailability, AvailabilityDef};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Entry {
    InclusionDirective {
        name: String,
        path: PathBuf
    },
    TypedefDecl {
        name: String,
        objc_type: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    EnumDecl {
        decls: Vec<EnumConstantDecl>,
        name: String,
        objc_type: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    VarDecl {
        name: String,
        objc_type: Typ,
        init_expr: Option<InitExpr>,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    StructDecl {
        fields: Vec<FieldDecl>,
        name: String,
        objc_type: Typ,
        platform_availability: Option<Vec<PlatformAvailability>>,
        #[serde(with = "AvailabilityDef")]
        availability: clang::Availability,
    },
    FunctionDecl {
        name: String,
        objc_type: Typ,
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
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum InitExpr {
    Value(InitValue),
    InitListExpr(InitListExpr),
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]

pub struct InitListExpr {
    pub values: Vec<InitValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjCMethodDecl {
    pub name: String,
    pub arguments: Vec<ParmDecl>,
    pub result_type: Typ,
    pub platform_availability: Option<Vec<PlatformAvailability>>,
    #[serde(with = "AvailabilityDef")]
    pub availability: clang::Availability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjCPropertyDecl {
    pub name: String,
    pub objc_type: Typ,
    pub attributes: Option<ObjCAttributes>,
    pub platform_availability: Option<Vec<PlatformAvailability>>,
    #[serde(with = "AvailabilityDef")]
    pub availability: clang::Availability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParmDecl {
    pub name: String,
    pub objc_type: Typ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateTypeParameter {
    pub name: String,
    pub constraint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumConstantDecl {
    pub name: String,
    pub value: Option<String>,
    pub objc_type: Typ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDecl {
    pub name: String,
    pub objc_type: Typ,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootEntry {
    pub root: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderFile {
    root: Vec<Entry>,
}
