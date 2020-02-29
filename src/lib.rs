pub mod branch;
pub mod config;
pub mod reader;

use std::path::Path;

use branch::Branch;
use config::Config;
use failure::Error;

pub fn run(config: Config) -> Result<(), Error> {
    let ignore_files = reader::read_ignore(&config.path);
    let branch = Branch::new((&Path::new(&config.path)).to_path_buf())?;

    let state = branch.read_children(&ignore_files)?;
    let mut branches = state.branches;
    let dir_count = state.dir_count;
    let file_count = state.file_count;

    branches.sort_by(|a, b| a.path.cmp(&b.path));

    // Output
    println!("{}", config.path);
    for (i, branch) in branches.iter().enumerate() {
        branch.println(i == branches.len() - 1);
    }
    println!("\n{} directories, {} files", dir_count, file_count);

    Ok(())
}
