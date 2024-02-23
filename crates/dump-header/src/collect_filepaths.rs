use std::collections::HashSet;
use std::path::PathBuf;

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
