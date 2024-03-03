use std::path::{Path, PathBuf};
use temp_dir::TempDir;

use crate::{
    headerfiletree::{HeaderFile, HeaderFileTree},
    parser::Parser,
};

static FRAMEWORKS: &[&str] = &["Foundation", "UIKit"];

pub struct Framework {
    name: String,
    root_header: PathBuf,
    header_file_tree: HeaderFileTree,
}

impl Framework {
    fn new(name: String, root_header: PathBuf, header_file_tree: HeaderFileTree) -> Self {
        Framework {
            name,
            root_header,
            header_file_tree,
        }
    }

    pub fn create_with_parser(
        name: &str,
        parser: &Parser,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let root_header = format!("#include <{}/{}.h>", name, name);
        let dir = TempDir::new().unwrap();
        let heder_file = dir.child("t.h");
        std::fs::write(&heder_file, root_header)?;
        let tu = parser.parse(&heder_file)?;
        let header_file_tree = HeaderFileTree::from_root_path(&heder_file, &tu);
        let framework = Self::new(name.to_string(), heder_file, header_file_tree);
        Ok(framework)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_dir(&self) -> &Path {
        self.root_header.parent().unwrap()
    }

    fn include(&self, path: &Path) -> bool {
        path.starts_with(self.root_dir())
    }

    pub fn iter(&self) -> impl Iterator<Item = &HeaderFile> {
        self.header_file_tree
            .iter()
            .filter(|hf| self.include(&hf.path))
    }
}
