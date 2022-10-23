use std::env;

pub fn get_file_path_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Usage: cargo run -- file.json");
    }

    let file_path = &args[1];
    file_path.to_string()
}
