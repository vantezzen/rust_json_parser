mod element;
mod file;
use file::read_file_characters;

mod parsers;
use parsers::parse_characters;

mod cli;
use cli::get_file_path_from_args;

fn main() {
    println!("Simple JSON Parser");

    let file_path = get_file_path_from_args();
    let mut chars = read_file_characters(file_path);
    chars.reverse();

    let contents = parse_characters(&mut chars);
    dbg!(contents);
}
