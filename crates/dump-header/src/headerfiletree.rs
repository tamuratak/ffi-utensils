use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use clang::source::File;

use crate::entity::{convert_entity, HeaderFile};

struct HeaderFileNode {
    file: Rc<RefCell<super::HeaderFile>>,
    children: Vec<Rc<RefCell<HeaderFileNode>>>,
}

impl HeaderFileNode {
    pub fn new(file: super::HeaderFile) -> Self {
        HeaderFileNode {
            file: Rc::new(RefCell::new(file)),
            children: vec![],
        }
    }
}

pub fn create_hedear_file_entry(root_entity: &clang::Entity, filename: &PathBuf) -> super::HeaderFile {
    let mut entries = vec![];
    root_entity.get_children().iter().for_each(|e| {
        if is_in_file(root_entity, filename) {
            if let Some(entry) = convert_entity(e) {
                entries.push(Box::new(entry));
            }
        }
    });
    super::HeaderFile {
        entries,
        path: filename.clone(),
    }
}

pub fn create_header_file_tree(root_entity: &clang::Entity) -> Option<HeaderFileNode> {
    let mut path_entry_hash_map: HashMap<PathBuf, HeaderFile> = HashMap::new();
    root_entity.get_children().iter().for_each(|e| {
        if let Some(path) = get_file_location_path(e) {
            if path_entry_hash_map.get(path);
        }
    });
    None
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
