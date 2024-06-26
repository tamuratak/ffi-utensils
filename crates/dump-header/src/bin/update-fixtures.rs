use std::path::PathBuf;

use anyhow::Result;
use clang::Clang;
use dump_header::{
    fixture::{Fixture, FixtureFile},
    headerfiletree::HeaderFile,
    parser::{self, ParserConfig},
};

fn main() -> Result<()> {
    clang_sys::load().unwrap();
    let clang = Clang::new().unwrap();
    let parser_config = ParserConfig {
        isysroot: None,
        lang: dump_header::cli::Lang::ObjC,
        std: None,
        target: None,
    };
    let parser = parser::Parser::from(&clang, parser_config);
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_dir = crate_root.join("tests").join("fixtures").join("objc");
    for fixture_path in std::fs::read_dir(fixture_dir)? {
        let fixture_path = fixture_path?.path();
        let mut fixture_file = FixtureFile::from(&fixture_path)?;
        let (tu, path) = parser.parse_content(fixture_file.fixture.source())?;
        let header_file_entry = HeaderFile::from_path(&path, &tu);
        let json = serde_json::to_string_pretty(&header_file_entry.entries)?;
        fixture_file.update(&Fixture::new(fixture_file.fixture.source(), &json))?;
    }
    Ok(())
}
