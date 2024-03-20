use crate::typ::Typ;

mod attributes;
mod availability;
mod entry;
mod vardecl;

use attributes::ObjCAttributes;
use availability::get_platform_availability;
pub use entry::{
    Entry, EnumConstantDecl, ObjCMethodDecl, ObjCPropertyDecl, ParmDecl, TemplateTypeParameter,
};

use self::vardecl::get_init_expr;

pub fn convert_entity(entity: &clang::Entity) -> Option<Entry> {
    let name = entity.get_name();
    let kind = entity.get_kind();
    let platform_availability = get_platform_availability(entity);
    let availability = entity.get_availability();
    match kind {
        clang::EntityKind::InclusionDirective => {
            let path: Option<std::path::PathBuf> = entity.get_file().map(|f| f.get_path());
            path.map(|path| Entry::InclusionDirective {
                name: name.unwrap(),
                path,
            })
        }
        clang::EntityKind::TypedefDecl => Some(Entry::TypedefDecl {
            name: name.unwrap(),
            ty: entity
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
                name: name.unwrap(),
                ty: Typ::from(entity.get_enum_underlying_type().unwrap()),
                platform_availability,
                availability,
            })
        }
        clang::EntityKind::VarDecl => {
            let init_expr = get_init_expr(entity);
            Some(Entry::VarDecl {
                name: name.unwrap(),
                ty: Typ::from(entity.get_type().unwrap()),
                init_expr,
                platform_availability,
                availability,
            })
        }
        clang::EntityKind::StructDecl => Some(Entry::StructDecl {
            name: name.unwrap(),
            fields: get_fields(entity),
            ty: Typ::from(entity.get_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::UnionDecl => Some(Entry::UnionDecl {
            name: name.unwrap(),
            is_anonymous: entity.is_anonymous_record_decl(),
            ty: Typ::from(entity.get_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::FunctionDecl => Some(Entry::FunctionDecl {
            name: name.unwrap(),
            ty: Typ::from(entity.get_type().unwrap()),
            arguments: get_arguments(entity),
            result_type: Typ::from(entity.get_result_type().unwrap()),
            platform_availability,
            availability,
        }),
        clang::EntityKind::ObjCInterfaceDecl
        | clang::EntityKind::ObjCCategoryDecl
        | clang::EntityKind::ObjCProtocolDecl => {
            let mut protocols: Vec<String> = vec![];
            let mut template_args: Vec<TemplateTypeParameter> = vec![];
            let mut superclass: String = Default::default();
            let mut class_name: String = Default::default();
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
                    clang::EntityKind::ObjCClassRef => {
                        class_name = e.get_name().unwrap();
                    }
                    clang::EntityKind::ObjCInstanceMethodDecl
                    | clang::EntityKind::ObjCClassMethodDecl => {
                        let arguments: Vec<ParmDecl> = get_arguments(entity);
                        let method = ObjCMethodDecl {
                            name: e.get_name().unwrap(),
                            arguments,
                            result_type: Typ::from(e.get_result_type().unwrap()),
                            optional: e.is_objc_optional(),
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
                        let attributes = e.get_objc_attributes().map(ObjCAttributes::from);
                        let property = ObjCPropertyDecl {
                            name: e.get_name().unwrap(),
                            objc_type: Typ::from(e.get_type().unwrap()),
                            optional: e.is_objc_optional(),
                            attributes,
                            platform_availability: get_platform_availability(e),
                            availability: e.get_availability(),
                        };
                        properties.push(property);
                    }
                    _ => {}
                });
            match kind {
                clang::EntityKind::ObjCInterfaceDecl => Some(Entry::ObjCInterfaceDecl {
                    name: name.unwrap(),
                    template_args,
                    superclass,
                    instance_methods,
                    class_methods,
                    properties,
                    protocols,
                    platform_availability,
                    availability,
                }),
                clang::EntityKind::ObjCCategoryDecl => Some(Entry::ObjCCategoryDecl {
                    name,
                    class_name,
                    instance_methods,
                    class_methods,
                    properties,
                    platform_availability,
                    availability,
                }),
                clang::EntityKind::ObjCProtocolDecl => Some(Entry::ObjCProtocolDecl {
                    name: name.unwrap(),
                    instance_methods,
                    class_methods,
                    properties,
                    platform_availability,
                    availability,
                }),
                _ => panic!("Invalid ObjC entity kind: {:?}", kind),
            }
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

fn get_fields(entity: &clang::Entity) -> Vec<Entry> {
    entity
        .get_children()
        .iter()
        .filter_map(|e| match e.get_kind() {
            clang::EntityKind::FieldDecl | clang::EntityKind::UnionDecl => convert_entity(e),
            _ => None,
        })
        .collect()
}
