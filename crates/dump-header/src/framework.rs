use clang::TranslationUnit;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

    pub fn root_header(&self) -> &Path {
        &self.root_header
    }

    pub fn iter(&self) -> impl Iterator<Item = &HeaderFile> {
        self.header_file_tree.iter()
    }
}

#[derive(Debug)]
pub struct FrameworkUnit<'a> {
    name: String,
    root_header: PathBuf,
    tu: TranslationUnit<'a>,
}

impl<'a> FrameworkUnit<'a> {
    fn new(name: String, root_header: PathBuf, tu: TranslationUnit<'a>) -> Self {
        FrameworkUnit {
            name,
            root_header,
            tu,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_header(&self) -> &Path {
        &self.root_header
    }

    pub fn with_parser(name: &str, parser: &'a Parser) -> Result<Self, Box<dyn std::error::Error>> {
        let root_header = format!("#include <{}/{}.h>", name, name);
        let tu = parser.parse_content(&root_header)?;
        let root_header = Self::get_root_header(&tu);
        let framework = Self::new(name.to_string(), root_header, tu);
        Ok(framework)
    }

    fn get_root_header(tu: &TranslationUnit<'a>) -> PathBuf {
        tu.get_entity()
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

    pub fn root_dir(&self) -> &Path {
        self.root_header.parent().unwrap()
    }

    fn include(&self, path: &Path) -> bool {
        path.starts_with(self.root_dir())
    }

    pub fn dump(&self) -> Framework {
        let root_header = self.root_header.clone();
        let header_file_tree =
            HeaderFileTree::from_root_header(&root_header, &self.tu, |path| self.include(path));
        Framework::new(self.name.clone(), root_header, header_file_tree)
    }
}
