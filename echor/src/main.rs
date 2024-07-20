use clap::Parser;

/// Rust echo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Do not print newline
    #[arg(short = 'n', long, default_value_t = false)]
    omit_newline: bool,

    /// Input text
    #[arg(value_name = "TEXT", required = true)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let ending = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), ending);
}
