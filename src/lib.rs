extern crate failure;

pub mod branch;
pub mod config;

use std::fs;

use branch::Branch;
use config::Config;

pub fn run(config: Config) -> Result<(), failure::Error> {
    let dir = fs::read_dir(&config.path)?;
    let mut branches: Vec<Branch> = vec![];
    let mut dir_count: u32 = 0;
    let mut file_count: u32 = 0;

    for entry in dir {
        let path = entry.unwrap().path();
        branches.push(Branch::new(path));
    }

    branches.sort_by(|a, b| a.name.cmp(&b.name));

    println!("{}", config.path);

    for (i, branch) in branches.iter().enumerate() {
        if i == branches.len() - 1 {
            println!("└──{}", branch.name);
        } else {
            println!("├── {}", branch.name);
        }
        if branch.is_dir {
            dir_count = dir_count + 1;
        } else {
            file_count = file_count + 1;
        }
    }

    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
