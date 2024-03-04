use std::path::PathBuf;

pub fn get_file_location_path(entity: &clang::Entity) -> Option<PathBuf> {
    entity
        .get_location()
        .map(|sl| PathBuf::from(sl.get_presumed_location().0))
}

pub fn is_in_file(entity: &clang::Entity, filename: &PathBuf) -> bool {
    get_file_location_path(entity)
        .map(|p| p == *filename)
        .unwrap_or(false)
}
