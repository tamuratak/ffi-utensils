use clang::TranslationUnit;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use temp_dir::TempDir;

use crate::{
    headerfiletree::{HeaderFile, HeaderFileTree},
    parser::Parser,
};

static FRAMEWORKS: &[&str] = &["Foundation", "UIKit"];

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug)]
pub struct FrameworkUnit<'a> {
    name: String,
    tu: TranslationUnit<'a>,
}

impl<'a> FrameworkUnit<'a> {
    fn new(name: String, tu: TranslationUnit<'a>) -> Self {
        FrameworkUnit { name, tu }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_parser(
        name: &str,
        parser: &'a Parser,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let root_header = format!("#include <{}/{}.h>", name, name);
        let dir = TempDir::new().unwrap();
        let heder_file = dir.child("t.h");
        std::fs::write(&heder_file, root_header)?;
        let tu = parser.parse(&heder_file)?;
        let framework = Self::new(name.to_string(), tu);
        Ok(framework)
    }

    pub fn root_header_file_path(&self) -> PathBuf {
        self.tu
            .get_entity()
            .get_children()
            .iter()
            .find_map(|entity| {
                if entity.get_kind() == clang::EntityKind::InclusionDirective {
                    entity.get_file().map(|f| f.get_path())
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn dump(&self) -> Framework {
        let root_header = self.root_header_file_path();
        let header_file_tree = HeaderFileTree::from_root_path(&root_header, &self.tu);
        Framework::new(self.name.clone(), root_header, header_file_tree)
    }
}
