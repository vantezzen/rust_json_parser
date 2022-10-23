use std::fs;

pub fn read_file_characters(file_path: String) -> String {
    println!("Reading file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}
