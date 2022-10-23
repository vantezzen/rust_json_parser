use std::env;
use std::fs;

mod parser;
use parser::parse_item;

mod element;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Usage: cargo run -- file.json");
    }

    let file_path = &args[1];
    println!("Reading file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut chars: Vec<_> = contents.chars().collect();
    chars.reverse();

    let contents = parse_item(&mut chars);
    dbg!(contents);
}
