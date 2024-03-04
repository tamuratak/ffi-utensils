#![allow(dead_code)]
use clang::{Clang, TranslationUnit};
use headerfiletree::HeaderFile;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::headerfiletree::HeaderFileTree;

mod entity;
mod framework;
mod headerfiletree;
mod parser;
mod typ;

// TODO
// - framework 
// - header file を作って parse して framework tree を生成する
// - framework の dependencies 
// - cli

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
    println!("{:?}", HeaderFileTree::from_root_path(&filename, &tu));
    call_save_to_file(&tu, &filename);
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

fn call_save_to_file(tu: &TranslationUnit, current_filename: &PathBuf) {
    let header_file_entry = HeaderFile::from_path(current_filename, tu);
    save_to_file(&header_file_entry, "point.json").unwrap();
}

fn save_to_file<T: Serialize>(data: &T, filename: &str) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(data)?;

    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
