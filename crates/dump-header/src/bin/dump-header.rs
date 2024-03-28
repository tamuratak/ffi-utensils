use std::path::PathBuf;

use anyhow::Result;
use clang::Clang;
use clap::{Parser, Subcommand};
use dump_header::{
    cli::{Lang, Std},
    framework::FrameworkUnit,
    headerfiletree::HeaderFile,
    parser::{self, ParserConfig},
};

// dump a single header file
// dump a single framework
// dump all the header files in a directory
// dump all the frameworks
// output to file, if not to stdout
// subcommands: dump, list functions, dependencies of framework, list headers..

// https://docs.rs/clap/latest/clap/_derive/index.html
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, value_enum, default_value_t = Lang::C)]
    lang: Lang,

    #[arg(long, value_enum)]
    std: Option<Std>,

    #[arg(long)]
    isysroot: Option<PathBuf>,

    #[arg(long)]
    target: Option<String>,

    /// Extra arguments to pass to clang
    #[arg(long)]
    clang_args: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
    /// dump a header file
    Dump {
        /// JSON file for output
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// The header file to dump
        file: PathBuf,
    },
    /// does framework stuff
    Framework {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// The framework to dump
        name: Option<String>,
    },
    /// print the AST of a file
    Ast { file: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    clang_sys::load().unwrap();
    let clang = Clang::new().unwrap();
    let parser_config = ParserConfig {
        isysroot: cli.isysroot.clone(),
        lang: cli.lang,
        std: cli.std,
        target: cli.target.clone(),
    };
    let parser = parser::Parser::from(&clang, parser_config);

    match &cli.command {
        Commands::Dump { output, file } => {
            let pwd = std::env::current_dir()?;
            let file = file
                .is_absolute()
                .then(|| file.clone())
                .unwrap_or(pwd.join(file));
            let tu = parser.parse(&file)?;
            let header_file_entry = HeaderFile::from_path(&file, &tu);
            if let Some(output) = output {
                header_file_entry.save(output)?;
            } else {
                println!("{}", serde_json::to_string_pretty(&header_file_entry)?);
            }
        }
        #[allow(unused_variables)]
        Commands::Framework { all, output, name } => {
            println!("{:?}", FrameworkUnit::with_parser("Foundation", &parser)?);
        }
        Commands::Ast { file } => {
            let tu = parser.parse(file)?;
            tu.get_entity().get_children().iter().for_each(|entity| {
                if entity.is_in_main_file() {
                    pretty_print_entity(entity, 0);
                }
            });
        }
    }
    Ok(())
}

fn pretty_print_entity(entity: &clang::Entity, depth: usize) {
    let spaces = "  ".repeat(depth);
    println!(
        "{}{:?} {:?}  {}",
        spaces,
        entity.get_name().unwrap_or("".to_string()),
        entity.get_kind(),
        entity
            .get_type()
            .map(|t| t.get_display_name())
            .unwrap_or("".to_string())
    );
    entity.get_children().iter().for_each(|entity| {
        pretty_print_entity(entity, depth + 1);
    });
}
