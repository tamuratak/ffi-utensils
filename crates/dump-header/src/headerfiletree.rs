use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use clang::source::File;

use crate::entity::convert_entity;

#[derive(Debug, Clone)]
pub struct HeaderFileNode {
    pub file: Rc<RefCell<super::HeaderFile>>,
    pub children: Vec<Rc<RefCell<HeaderFileNode>>>,
}

impl HeaderFileNode {
    pub fn new(file: super::HeaderFile) -> Self {
        HeaderFileNode {
            file: Rc::new(RefCell::new(file)),
            children: vec![],
        }
    }
}

pub fn create_hedear_file_entry(
    root_entity: &clang::Entity,
    filename: &PathBuf,
) -> super::HeaderFile {
    let mut entries = vec![];
    root_entity.get_children().iter().for_each(|e| {
        if is_in_file(root_entity, filename) {
            if let Some(entry) = convert_entity(e) {
                entries.push(entry);
            }
        }
    });
    super::HeaderFile {
        entries,
        path: filename.clone(),
    }
}

pub fn create_header_file_tree(
    root_entity: &clang::Entity,
    root_filepath: &PathBuf,
) -> Option<Rc<RefCell<HeaderFileNode>>> {
    let mut path_entry_hash_map: HashMap<PathBuf, Rc<RefCell<HeaderFileNode>>> = HashMap::new();
    root_entity.get_children().iter().for_each(|e| {
        if let Some(path) = get_file_location_path(e) {
            if path_entry_hash_map.get(&path).is_none() {
                let header_file_node = HeaderFileNode::new(create_hedear_file_entry(e, &path));
                path_entry_hash_map.insert(path.clone(), Rc::new(RefCell::new(header_file_node)));
            }
        }
    });
    let root_node = path_entry_hash_map.get(root_filepath);

    path_entry_hash_map
        .iter()
        .for_each(|(_, header_file_node)| {
            header_file_node
                .borrow_mut()
                .file
                .borrow()
                .get_include_directives()
                .iter()
                .for_each(|(_, path)| {
                    if let Some(child) = path_entry_hash_map.get(path) {
                        header_file_node.borrow_mut().children.push(child.clone());
                    }
                })
        });
    root_node.map(|node| node.clone())
}

pub fn traverse<'tu>(entity: &clang::Entity<'tu>, filename: &PathBuf) -> Vec<File<'tu>> {
    let mut vec = vec![];
    entity.get_children().iter().for_each(|e| {
        if clang::EntityKind::InclusionDirective == e.get_kind() && is_in_file(e, filename) {
            e.get_file().map(|f| vec.push(f));
        }
    });
    vec
}

fn is_in_file(entity: &clang::Entity, filename: &PathBuf) -> bool {
    if let Some(sl) = entity.get_location() {
        if let Some(f) = sl.get_file_location().file {
            if PathBuf::from(f.get_path()) == *filename {
                return true;
            }
        }
    }
    false
}

fn get_file_location_path(entity: &clang::Entity) -> Option<PathBuf> {
    entity
        .get_location()
        .map(|sl| sl.get_file_location().file.map(|f| f.get_path()))
        .flatten()
}
