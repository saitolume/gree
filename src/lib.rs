pub mod branch;
pub mod config;
pub mod reader;

use std::io::{self, BufWriter, Write};
use std::path::Path;

use branch::Branch;
use config::Config;
use failure::Error;

pub fn run(config: Config) -> Result<(), Error> {
    let ignore_files = reader::read_ignore(&config.path);
    let branch = Branch::new((&Path::new(&config.path)).to_path_buf())?;
    let (mut branches, dir_count, file_count) = branch.read_children(&ignore_files)?;

    branches.sort_by(|a, b| a.path.cmp(&b.path));

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    writeln!(output, "{}", config.path).unwrap();
    for (i, branch) in branches.iter().enumerate() {
        writeln!(output, "{}", branch.row(i == branches.len() - 1)).unwrap();
    }
    writeln!(output, "\n{} directories, {} files", dir_count, file_count).unwrap();

    Ok(())
}
