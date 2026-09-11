use std::{env, fs, process};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("file path required");
        process::exit(2);
    });
    let bytes = fs::read(path).unwrap_or_else(|error| {
        eprintln!("cannot read file: {error}");
        process::exit(2);
    });
    println!("{}", blake3::hash(&bytes).to_hex());
}
