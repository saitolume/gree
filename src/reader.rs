use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_ignore() -> Result<Vec<String>, Error> {
    let mut ignore_file_names: Vec<String> = vec![];

    for line in BufReader::new(File::open(".gitignore")?).lines() {
        let file_name = Path::new(&line?).file_name().unwrap().to_str().unwrap().to_string();
        ignore_file_names.push(file_name);
    }
    Ok(ignore_file_names)
}
