use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

pub fn run() -> Result<()> {
    let args = Args::parse();
    let mut num = 0;
    for filename in args.files {
        match open(&filename) {
            Ok(reader) => {
                for line in reader.lines() {
                    let line = line?;
                    if args.number_lines {
                        num += 1;
                        println!("{num:6}\t{line}");
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            num += 1;
                            println!("{num:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
            Err(err) => eprintln!("{filename}: {err}"),
        }
    }
    Ok(())
}

/// Rust cat
#[derive(Parser, Debug)]
#[command(version, about, author,long_about = None)]
struct Args {
    /// Number lines
    #[arg(
        short = 'n',
        long = "number",
        default_value_t = false,
        conflicts_with("number-nonblank")
    )]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(
        short = 'b',
        long = "number-nonblank",
        group = "number-nonblank",
        default_value_t = false
    )]
    number_nonblank_lines: bool,

    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
