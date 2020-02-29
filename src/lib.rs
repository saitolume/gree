pub mod branch;
pub mod config;
pub mod reader;

use std::fs;

use branch::Branch;
use config::Config;
use failure::Error;

pub fn run(config: Config) -> Result<(), Error> {
    let dir = fs::read_dir(&config.path)?;
    let ignore_files = reader::read_ignore(&config.path);
    let mut branches: Vec<Branch> = vec![];
    let mut dir_count: u32 = 0;
    let mut file_count: u32 = 0;

    for entry in dir {
        let branch = Branch::new(entry?.path());
        if ignore_files.contains(&branch.name) {
            continue;
        }
        if branch.is_dir {
            let state = branch.child_node(&ignore_files)?;
            branches.extend(state.branches);
            dir_count += state.dir_count;
            file_count += state.file_count;
        }
        if branch.is_dir {
            dir_count += 1
        } else {
            file_count += 1
        }
        branches.push(branch);
    }

    branches.sort_by(|a, b| a.path.cmp(&b.path));

    // Output
    println!("{}", config.path);
    for (i, branch) in branches.iter().enumerate() {
        branch.println(i == branches.len() - 1);
    }
    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
