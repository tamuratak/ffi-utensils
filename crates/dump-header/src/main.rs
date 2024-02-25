use clang::{Clang, Index};
use entity::{convert_entity, HeaderFile};
use headerfiletree::create_hedear_file_entry;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::headerfiletree::create_header_file_tree;

mod entity;
mod headerfiletree;
mod typ;

use headerfiletree::get_file_location_path;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

// TODO
// - Add memo to Typ::from. recursive type causes segfault.

fn main() -> Result<(), BoxError> {
    let args: Vec<String> = env::args().collect();
    let file_arg = &args[1];
    clang_sys::load()?;
    let clang = Clang::new()?;
    let index = Index::new(&clang, true, true);
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(file_arg);
    let tu = index
        .parser(filename.clone())
        .detailed_preprocessing_record(true)
        .incomplete(true)
        .skip_function_bodies(true)
        .keep_going(true)
        // .single_file_parse(true)
        .include_attributed_types(true)
        .visit_implicit_attributes(true)
        // .ignore_non_errors_from_included_files(true)
        .retain_excluded_conditional_blocks(true)
        .arguments(&[
            "-x",
            "objective-c",
            "--target=arm64-apple-darwin22.6.0",
            "-Wall",
            "-Wextra",
            "-fobjc-arc",
            "-fobjc-arc-exceptions",
            "-fobjc-abi-version=2", // 3??
            // "-fparse-all-comments",
            // TODO: "-fretain-comments-from-system-headers"
            "-fapinotes",
            "-isysroot",
            "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/",
            // See ClangImporter.cpp and Foundation/NSObjCRuntime.h
            "-D",
            "__SWIFT_ATTR_SUPPORTS_SENDABLE_DECLS=1",
        ])
        .parse()
        .unwrap();
    tu.get_entity().get_children().iter().for_each(|entity| {
        if entity.is_in_main_file() {
            pretty_print_entity(entity, 0);
        }
    });
    println!("{:?}", headerfiletree::traverse(&tu.get_entity(), &filename));
    println!("{:?}", create_header_file_tree(&tu.get_entity(), &filename));
    call_save_to_file(&tu.get_entity(), &filename);
    Ok(())
}

fn pretty_print_entity(entity: &clang::Entity, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
    println!(
        "{:?} {:?} {:?} {:?}",
        entity.get_name(),
        entity.get_kind(),
        entity.get_type(),
        get_file_location_path(entity)
    );
    entity.get_children().iter().for_each(|entity| {
        pretty_print_entity(entity, depth + 1);
    });
}

fn call_save_to_file(root: &clang::Entity, current_filename: &PathBuf) {
    let header_file_entry = create_hedear_file_entry(&root.get_children(), current_filename);
    save_to_file(&header_file_entry, "point.json").unwrap();
}

fn save_to_file<T: Serialize>(data: &T, filename: &str) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(data)?;

    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
