use std::{env, fs};
fn main() {
    let path = env::args_os().nth(1).expect("path");
    let bytes = fs::read(path).expect("read");
    println!("{}", blake3::hash(&bytes).to_hex());
}
