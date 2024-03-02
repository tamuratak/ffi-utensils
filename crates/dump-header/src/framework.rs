use std::path::{Path, PathBuf};

use crate::headerfiletree::{HeaderFile, HeaderFileTree};

pub struct Framework {
    name: String,
    root_header: PathBuf,
    header_file_tree: HeaderFileTree,
}

impl Framework {
    pub fn new(name: String, root_header: PathBuf, header_file_tree: HeaderFileTree) -> Self {
        Framework {
            name,
            root_header,
            header_file_tree,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_dir(&self) -> &Path {
        self.root_header.parent().unwrap()
    }

    fn include(&self, path: &PathBuf) -> bool {
        path.starts_with(self.root_dir())
    }

    pub fn iter(&self) -> impl Iterator<Item = &HeaderFile> {
        self.header_file_tree
            .iter()
            .filter_map(|hf| self.include(&hf.path).then_some(hf))
    }
}
