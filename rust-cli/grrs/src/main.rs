use std::io::{BufReader, BufRead};
use std::fs::File;
use::clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    //println!("pattern: {}, path: {}",args.pattern, args.path.display());
    let file = File::open(args.path).expect("could not read file");
    let mut reader = BufReader::new(file);
    for line in reader.lines() {
        if line.unwrap_or_default().contains(&args.pattern) {
        }
    }
}
