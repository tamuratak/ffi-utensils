use std::collections::HashSet;
use std::path::PathBuf;

use clang::source::File;

pub fn collect_filepaths(entity: &clang::Entity) -> HashSet<PathBuf> {
    let mut filepaths = HashSet::new();
    entity.get_children().iter().for_each(|entity| {
        let path = entity.get_location().map(|sl| sl.get_file_location().file.map(|f| f.get_path()));
        if let Some(Some(path)) = path {
            filepaths.insert(PathBuf::from(path));
        }
    });
    filepaths
}

pub fn traverse<'tu>(entity: &clang::Entity<'tu>, filename: &PathBuf) -> Vec<File<'tu>> {
    let mut vec = vec![];
    entity.get_children().iter().for_each(|e| {
        if clang::EntityKind::InclusionDirective == e.get_kind() {
            if let Some(sl) = e.get_location() {
                if let Some(f) = sl.get_file_location().file {
                    if PathBuf::from(f.get_path()) == *filename {
                        e.get_file().map(|f| vec.push(f));
                    }
                }
            }
        }
    });
    vec
}

