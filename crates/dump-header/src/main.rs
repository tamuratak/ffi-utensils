#![allow(dead_code)]
use clang::Clang;
use headerfiletree::HeaderFile;
use std::env;
use std::path::Path;

use crate::framework::FrameworkUnit;
use crate::headerfiletree::HeaderFileTree;

mod entity;
mod framework;
mod headerfiletree;
mod parser;
mod typ;
mod utils;

// TODO
// - framework
//   - sdk path
// - framework's dependencies
// - cli with clap
// - add debug print

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_arg = &args[1];
    clang_sys::load()?;
    let clang = Clang::new()?;
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(file_arg);
    let parser = parser::Parser::from(&clang);
    let tu = parser.parse(&filename).unwrap();
    tu.get_entity().get_children().iter().for_each(|entity| {
        if entity.is_in_main_file() {
            pretty_print_entity(entity, 0);
        }
    });
    println!(
        "{:?}",
        HeaderFileTree::from_root_header(&filename, &tu, |_| true)
    );
    println!(
        "{:?}",
        FrameworkUnit::with_parser("Foundation", &parser)?
    );
    let header_file_entry = HeaderFile::from_path(&filename, &tu);
    header_file_entry.save(Path::new("./point.json")).unwrap();
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
        ""
    );
    entity.get_children().iter().for_each(|entity| {
        pretty_print_entity(entity, depth + 1);
    });
}
