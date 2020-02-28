extern crate failure;

pub mod branch;
pub mod config;
pub mod reader;

use std::fs;

use branch::Branch;
use config::Config;

pub fn run(config: Config) -> Result<(), failure::Error> {
    let dir = fs::read_dir(&config.path)?;
    let ignore_files = reader::read_ignore().unwrap();
    let mut branches: Vec<Branch> = vec![];
    let mut dir_count: u32 = 0;
    let mut file_count: u32 = 0;

    for entry in dir {
        let path = entry.unwrap().path();
        let branch = Branch::new(path);

        let contains = ignore_files.contains(&branch.name);
        if contains { continue }

        if branch.is_dir { dir_count += 1 } else { file_count += 1 }
        branches.push(branch);
    }

    branches.sort_by(|a, b| a.name.cmp(&b.name));

    // Output
    println!("{}", config.path);
    for (i, branch) in branches.iter().enumerate() {
        branch.println(i == branches.len() - 1);
    }
    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
