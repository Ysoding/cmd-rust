use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::{builder::PossibleValue, value_parser, ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

/// Rust find
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search path(s)
    #[arg(default_value = ".", value_name = "PATH")]
    paths: Vec<String>,

    /// Names
    #[arg(short = 'n', long, value_name = "NAME", value_parser = Regex::new, action = ArgAction::Append, num_args = 0..)]
    name: Vec<Regex>,

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser = value_parser!(EntryType),
        action = ArgAction::Append,
        num_args = 0..
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn main() {
    let args = Args::parse();
    run(args).unwrap();
}

fn run(args: Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        args.name.is_empty()
            || args
                .name
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
