use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Inputs file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(short = 'n', long, default_value = "10", value_parser=clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Number of bytes
    #[arg(short = 'c', long, value_name = "BYTES", conflicts_with("lines"), value_parser=clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    run(args).unwrap();
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" },);
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
