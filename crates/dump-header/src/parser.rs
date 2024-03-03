use std::path::PathBuf;

use clang::{Clang, Index, SourceError, TranslationUnit};

pub struct Parser<'a> {
    index: Index<'a>,
}

impl<'a> Parser<'a> {
    pub fn from(clang: &'a Clang) -> Self {
        let index = Index::new(clang, true, true);
        Self { index }
    }

    pub fn parse(&'a self, filename: &PathBuf) -> Result<TranslationUnit<'a>, SourceError> {
        let tu = self.index
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
            .parse();
        tu
    }
}
