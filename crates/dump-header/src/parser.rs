use std::path::{Path, PathBuf};

use clang::{Clang, Index, SourceError, TranslationUnit};
use clap::ValueEnum;
use temp_dir::TempDir;

use crate::cli::{Lang, Std};

pub struct Parser<'a> {
    index: Index<'a>,
    config: ParserConfig,
}

pub struct ParserConfig {
    pub isysroot: Option<PathBuf>,
    pub lang: Lang,
    pub std: Option<Std>,
    pub target: Option<String>,
}

impl<'a> Parser<'a> {
    pub fn from(clang: &'a Clang, config: ParserConfig) -> Self {
        let index = Index::new(clang, true, true);
        Self { index, config }
    }

    // https://clang.llvm.org/docs/UsersManual.html
    // https://clang.llvm.org/docs/CommandGuide/clang.html
    // https://clang.llvm.org/docs/ClangCommandLineReference.html
    pub fn parse(&'a self, filename: &Path) -> Result<TranslationUnit<'a>, SourceError> {
        let mut args = vec![];
        args.push("-x");
        let lang = match self.config.lang {
            Lang::C => "c",
            Lang::ObjC => "objective-c",
        };
        args.push(lang);
        if let Some(target) = &self.config.target {
            args.push("-target");
            args.push(target.as_str());
        }
        let std_name = self
            .config
            .std
            .as_ref()
            .and_then(|std| std.to_possible_value())
            .map(|v| v.get_name().to_string());
        if let Some(std_name) = &std_name {
            args.push("-std");
            args.push(std_name);
        };
        if let Some(isysroot) = &self.config.isysroot {
            args.push("-isysroot");
            args.push(isysroot.to_str().unwrap());
        } else if cfg!(target_os = "macos") {
            // TODO: Use https://crates.io/crates/apple-sdk
            args.push("-isysroot");
            args.push("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/");
        }
        if let Lang::ObjC = self.config.lang {
            args.extend(vec![
                "-fobjc-arc",
                "-fobjc-arc-exceptions",
                "-fobjc-abi-version=2",
            ]);
        }
        args.extend(vec![
            "-Wall",
            "-Wextra",
            "-fapinotes",
            // See ClangImporter.cpp and Foundation/NSObjCRuntime.h
            "-D",
            "__SWIFT_ATTR_SUPPORTS_SENDABLE_DECLS=1",
        ]);
        if std::env::var("DEBUG").is_ok() {
            eprintln!("Parser args: {:#?}", args);
        }
        let tu = self
            .index
            .parser(filename)
            .detailed_preprocessing_record(true)
            .incomplete(true)
            .skip_function_bodies(true)
            .keep_going(true)
            // .single_file_parse(true)
            .include_attributed_types(true)
            .visit_implicit_attributes(true)
            // .ignore_non_errors_from_included_files(true)
            .retain_excluded_conditional_blocks(true)
            .arguments(&args)
            .parse();
        tu
    }

    pub fn parse_content(
        &'a self,
        content: &str,
    ) -> Result<(TranslationUnit<'a>, PathBuf), Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let dir = TempDir::new().unwrap();
        let heder_file = dir.child("t.h");
        std::fs::write(&heder_file, content)?;
        Ok((self.parse(&heder_file)?, heder_file))
    }
}
