use std::collections::HashMap;
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderFileTree {
    root_filepath: PathBuf,
    path_entry_hash_map: HashMap<PathBuf, HeaderFile>,
}

impl HeaderFileTree {
    pub fn new(root_filepath: &PathBuf) -> Self {
        let path_entry_hash_map: HashMap<PathBuf, HeaderFile> = HashMap::new();
        HeaderFileTree {
            root_filepath: root_filepath.clone(),
            path_entry_hash_map,
        }
    }

    pub fn get(&self, path: &PathBuf) -> Option<HeaderFileNode> {
        self.path_entry_hash_map
            .get(path)
            .map(|n| HeaderFileNode::new(n, &self.path_entry_hash_map))
    }

    pub fn insert(&mut self, file: HeaderFile) {
        self.path_entry_hash_map
            .insert(file.path.clone(), file);
    }

    pub fn get_root(&self) -> Option<HeaderFileNode> {
        self.get(&self.root_filepath)
    }
}

#[derive(Debug, Clone)]
pub struct HeaderFileNode<'a> {
    header_file: &'a HeaderFile,
    path_entry_hash_map: &'a HashMap<PathBuf, HeaderFile>,
}

impl<'a> HeaderFileNode<'a> {
    pub fn new(
        header_file: &'a HeaderFile,
        path_entry_hash_map: &'a HashMap<PathBuf, HeaderFile>,
    ) -> Self {
        HeaderFileNode {
            header_file,
            path_entry_hash_map,
        }
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.header_file.entries
    }

    pub fn path(&self) -> &PathBuf {
        &self.header_file.path
    }

    pub fn get_children(&self) -> Vec<HeaderFileNode> {
        self.header_file
            .get_include_directives()
            .iter()
            .filter_map(|(_, path)| self.path_entry_hash_map.get(path))
            .map(|entry| HeaderFileNode::new(entry, self.path_entry_hash_map))
            .collect()
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
) -> HeaderFileTree {
    let mut header_file_tree = HeaderFileTree::new(root_filepath);
    root_entity.get_children().iter().for_each(|e| {
        if let Some(path) = get_file_location_path(e) {
            if header_file_tree.get(&path).is_none() {
                let header_file = create_hedear_file_entry(&root_entity.get_children(), &path);
                header_file_tree.insert(header_file);
            }
        }
    });
    header_file_tree
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
