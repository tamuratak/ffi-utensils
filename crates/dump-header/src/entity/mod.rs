use crate::typ::Typ;

mod attributes;
mod availability;
mod entry;
mod vardecl;

use attributes::ObjCAttributes;
use availability::get_platform_availability;
pub use entry::{Entry, EnumConstantDecl, ObjCMethodDecl, ObjCPropertyDecl, ParmDecl, TemplateTypeParameter, HeaderFile};

use self::vardecl::get_init_expr;


pub fn convert_entity(entity: &clang::Entity) -> Option<Entry> {
    let name0 = entity.get_name();
    if let None = name0 {
        return None;
    }
    let name = name0.unwrap();
    let kind = entity.get_kind();
    let platform_availability = get_platform_availability(entity);
    let availability = entity.get_availability();
    match kind {
        clang::EntityKind::InclusionDirective => {
            let path: Option<std::path::PathBuf> = entity.get_file().map(|f| f.get_path());
            if let Some(path) = path {
                Some(Entry::InclusionDirective {
                    name,
                    path
                })
            } else {
                None
            }
        },
        clang::EntityKind::TypedefDecl => Some(Entry::TypedefDecl {
            name,
            objc_type: entity
                .get_typedef_underlying_type()
                .map(|t| Typ::from(t))
                .unwrap(),
            platform_availability,
            availability,
        }),
        clang::EntityKind::EnumDecl => {
            let mut decls: Vec<EnumConstantDecl> = vec![];
            entity.get_children().iter().for_each(|e| {
                if let clang::EntityKind::EnumConstantDecl = e.get_kind() {
                    let value = if let Some(child) = e.get_child(0) {
                        match child.evaluate() {
                            Some(clang::EvaluationResult::SignedInteger(value)) => {
                                Some(value.to_string())
                            }
                            Some(clang::EvaluationResult::UnsignedInteger(value)) => {
                                Some(value.to_string())
                            }
                            _ => None,
                        }
                    } else {
                        None
                    };
                    decls.push(EnumConstantDecl {
                        name: e.get_name().unwrap(),
                        value,
                        objc_type: Typ::from(e.get_type().unwrap()),
                    });
                }
            });
            if decls.is_empty() {
                return None;
            }
            Some(Entry::EnumDecl {
                decls,
                name,
                objc_type: Typ::from(entity.get_enum_underlying_type().unwrap()),
                platform_availability,
                availability,
            })
        }
        clang::EntityKind::VarDecl => {
            let init_expr = get_init_expr(entity);
            Some(Entry::VarDecl {
                name,
                objc_type: Typ::from(entity.get_type().unwrap()),
                init_expr,
                platform_availability,
                availability,
            })
        },
        clang::EntityKind::StructDecl => Some(Entry::StructDecl {
            name,
            objc_type: Typ::from(entity.get_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::UnionDecl => Some(Entry::UnionDecl {
            name,
            is_anonymous: entity.is_anonymous_record_decl(),
            objc_type: Typ::from(entity.get_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::FunctionDecl => Some(Entry::FunctionDecl {
            name,
            objc_type: Typ::from(entity.get_type().unwrap()),
            arguments: get_arguments(entity),
            result_type: Typ::from(entity.get_result_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::ObjCInterfaceDecl => {
            let mut protocols: Vec<String> = vec![];
            let mut template_args: Vec<TemplateTypeParameter> = vec![];
            let mut superclass: String = Default::default();
            let mut instance_methods: Vec<ObjCMethodDecl> = vec![];
            let mut class_methods: Vec<ObjCMethodDecl> = vec![];
            let mut properties: Vec<ObjCPropertyDecl> = vec![];
            entity
                .get_children()
                .iter()
                .for_each(|e| match e.get_kind() {
                    clang::EntityKind::ObjCProtocolRef => {
                        protocols.push(e.get_name().unwrap());
                    }
                    clang::EntityKind::TemplateTypeParameter => {
                        let constraint = e.get_child(0).map(|x| x.get_name().unwrap());
                        template_args.push(TemplateTypeParameter {
                            name: e.get_name().unwrap(),
                            constraint,
                        });
                    }
                    clang::EntityKind::ObjCSuperClassRef => {
                        superclass = e.get_name().unwrap();
                    }
                    clang::EntityKind::ObjCInstanceMethodDecl
                    | clang::EntityKind::ObjCClassMethodDecl => {
                        let arguments: Vec<ParmDecl> = get_arguments(entity);
                        let method = ObjCMethodDecl {
                            name: e.get_name().unwrap(),
                            arguments,
                            result_type: Typ::from(e.get_result_type().unwrap()),
                            platform_availability: get_platform_availability(e),
                            availability: e.get_availability(),
                        };
                        if let clang::EntityKind::ObjCInstanceMethodDecl = e.get_kind() {
                            instance_methods.push(method);
                        } else {
                            class_methods.push(method);
                        }
                    }
                    clang::EntityKind::ObjCPropertyDecl => {
                        let attributes = e.get_objc_attributes().map(|a| ObjCAttributes::from(a));
                        let property = ObjCPropertyDecl {
                            name: e.get_name().unwrap(),
                            objc_type: Typ::from(e.get_type().unwrap()),
                            attributes,
                            platform_availability: get_platform_availability(e),
                            availability: e.get_availability(),
                        };
                        properties.push(property);
                    
                    }
                    _ => {}
                });
            Some(Entry::ObjCInterfaceDecl {
                name,
                template_args,
                superclass,
                instance_methods,
                class_methods,
                properties,
                protocols,
                platform_availability,
                availability,
            })
        }
        _ => None,
    }
}

fn get_arguments(entity: &clang::Entity) -> Vec<ParmDecl> {
    let mut arguments: Vec<ParmDecl> = vec![];
    if let Some(args) = entity.get_arguments() {
        args.iter().for_each(|arg| {
            if let clang::EntityKind::ParmDecl = arg.get_kind() {
                    arguments.push(ParmDecl {
                        name: arg.get_name(),
                        objc_type: Typ::from(arg.get_type().unwrap()),
                    });
            }
        });
    }
    arguments
}

/*
fn get_fields(entity: &clang::Entity) -> Vec<FieldDecl> {
    entity
        .get_children()
        .iter()
        .filter_map(|e| {
            if let clang::EntityKind::FieldDecl = e.get_kind() {
                Some(FieldDecl {
                    name: e.get_name().unwrap(),
                    objc_type: Typ::from(e.get_type().unwrap()),
                })
            } else {
                None
            }
        })
        .collect()
}
*/