use std::fs::{File};
use::clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    if !args.path.exists() {
        eprintln!("The file at '{}' does not exist", &args.path.display());
        return;
    }
    if args.pattern.is_empty() {
        eprintln!("No pattern was supplied, see --help");
        return;
    }
    let file = File::open(args.path).expect("could not read file");
    let matches = grrs::find_matches(&file, &args.pattern).unwrap();
    if matches.len() < 1 {
        return;
    }
    for line in matches {
        println!("{}", line);
    }
}
