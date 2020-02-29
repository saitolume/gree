use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_ignore(root_path: &str) -> Vec<String> {
    let mut ignore_file_names: Vec<String> = vec![];
    let gitignore_path = match root_path {
        ".." | "." => "/.gitignore",
        _ => ".gitignore",
    };

    if let Ok(file) = fs::File::open(Path::new(&(root_path.to_owned() + gitignore_path))) {
        for line in BufReader::new(file).lines() {
            let file_name = Path::new(&line.expect(""))
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            ignore_file_names.push(file_name);
        }
    }

    ignore_file_names
}
