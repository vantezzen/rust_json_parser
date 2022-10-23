use std::fs;

pub fn read_file_characters(file_path: String) -> Vec<char> {
    println!("Reading file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chars: Vec<_> = contents.chars().collect();
    chars
}
