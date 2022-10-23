mod element;
mod file;
use file::read_file_characters;

mod parsers;
use parsers::parse_string;

mod cli;
use cli::get_file_path_from_args;

fn main() {
    println!("Simple JSON Parser");

    let file_path = get_file_path_from_args();
    let string_contents = read_file_characters(file_path);
    
    let contents = parse_string(string_contents);
    dbg!(contents);
}
