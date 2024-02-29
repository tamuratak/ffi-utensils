use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use clang::source::File;
use serde::{Deserialize, Serialize};

use crate::entity::{convert_entity, Entry};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFile {
    pub entries: Vec<Entry>,
    pub path: PathBuf,
}

impl HeaderFile {
    pub fn new(entries: Vec<Entry>, path: PathBuf) -> Self {
        HeaderFile { entries, path }
    }

    pub fn get_include_directives(&self) -> Vec<(String, PathBuf)> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::InclusionDirective { name, path } => Some((name.clone(), path.clone())),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct HeaderFileNode {
    pub header_file: Rc<RefCell<HeaderFile>>,
    pub children: Vec<Rc<RefCell<HeaderFileNode>>>,
}

impl HeaderFileNode {
    pub fn new(file: HeaderFile) -> Self {
        HeaderFileNode {
            header_file: Rc::new(RefCell::new(file)),
            children: vec![],
        }
    }
}

pub fn create_hedear_file_entry(
    all_entries: &Vec<clang::Entity>,
    filename: &PathBuf,
) -> HeaderFile {
    let mut entries = vec![];
    all_entries.iter().for_each(|e| {
        if is_in_file(e, filename) {
            if let Some(entry) = convert_entity(e) {
                entries.push(entry);
            }
        }
    });
    HeaderFile::new(entries, filename.clone())
}

pub fn create_header_file_tree(
    root_entity: &clang::Entity,
    root_filepath: &PathBuf,
) -> Option<Rc<RefCell<HeaderFileNode>>> {
    let mut path_entry_hash_map: HashMap<PathBuf, Rc<RefCell<HeaderFileNode>>> = HashMap::new();
    root_entity.get_children().iter().for_each(|e| {
        if let Some(path) = get_file_location_path(e) {
            if path_entry_hash_map.get(&path).is_none() {
                let header_file_node = HeaderFileNode::new(create_hedear_file_entry(
                    &root_entity.get_children(),
                    &path,
                ));
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
                .header_file
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
    get_file_location_path(entity)
        .map(|p| p == *filename)
        .unwrap_or(false)
}

pub fn get_file_location_path(entity: &clang::Entity) -> Option<PathBuf> {
    entity
        .get_location()
        .map(|sl| sl.get_file_location().file.map(|f| f.get_path()))
        .flatten()
}
