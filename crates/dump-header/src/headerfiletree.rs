use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use clang::TranslationUnit;
use serde::{Deserialize, Serialize};

use crate::entity::{convert_entity, Entry};
use crate::utils::{get_file_location_path, is_in_file};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFile {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
}

impl HeaderFile {
    pub fn new(path: PathBuf, entries: Vec<Entry>) -> Self {
        HeaderFile { entries, path }
    }

    pub fn from_path(path: &PathBuf, tu: &TranslationUnit) -> Self {
        let mut entries = vec![];
        tu.get_entity().get_children().iter().for_each(|entity| {
            if is_in_file(entity, path) {
                if let Some(entry) = convert_entity(entity) {
                    entries.push(entry);
                }
            }
        });
        Self::new(path.clone(), entries)
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

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFileTree {
    root_path: PathBuf,
    path_entry_hash_map: HashMap<PathBuf, HeaderFile>,
}

impl HeaderFileTree {
    fn new(root_filepath: &Path) -> Self {
        let path_entry_hash_map: HashMap<PathBuf, HeaderFile> = HashMap::new();
        HeaderFileTree {
            root_path: root_filepath.to_path_buf(),
            path_entry_hash_map,
        }
    }

    pub fn from_root_header<F>(root_header: &Path, tu: &TranslationUnit, include: F) -> Self
    where
        F: Fn(&Path) -> bool,
    {
        let mut tree = Self::new(root_header);
        tu.get_entity().get_children().iter().for_each(|entity| {
            if let Some(header_file_path) = get_file_location_path(entity) {
                if tree.get(&header_file_path).is_none() && include(&header_file_path) {
                    if std::env::var("DEBUG").is_ok() {
                        eprintln!("Adding header file: {:?}", header_file_path)
                    }
                    let header_file = HeaderFile::from_path(&header_file_path, tu);
                    tree.insert(header_file);
                }
            }
        });
        tree
    }

    pub fn get(&self, path: &PathBuf) -> Option<HeaderFileNode> {
        self.path_entry_hash_map
            .get(path)
            .map(|hf| HeaderFileNode::new(hf, &self.path_entry_hash_map))
    }

    fn insert(&mut self, file: HeaderFile) {
        self.path_entry_hash_map.insert(file.path.clone(), file);
    }

    pub fn iter(&self) -> impl Iterator<Item = &HeaderFile> {
        self.path_entry_hash_map.values()
    }

    pub fn get_root(&self) -> Option<HeaderFileNode> {
        self.get(&self.root_path)
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
            .map(|hf| HeaderFileNode::new(hf, self.path_entry_hash_map))
            .collect()
    }
}
