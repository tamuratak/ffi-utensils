use std::path::PathBuf;

use clang::Clang;
use dump_header::{
    fixture::FixtureFile,
    headerfiletree::HeaderFile,
    parser::{self, ParserConfig},
};

use pretty_assertions::assert_eq;

#[test]
fn test_objc_parsing() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    clang_sys::load()?;
    let clang = Clang::new()?;
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
        let fixture_file = FixtureFile::from(&fixture_path)?;
        let (tu, path) = parser.parse_content(&(fixture_file.fixture.source()))?;
        let header_file_entry = HeaderFile::from_path(&path, &tu);
        let json = serde_json::to_string_pretty(&header_file_entry.entries)?;
        assert_eq!(fixture_file.fixture.json(), json);
    }
    Ok(())
}
