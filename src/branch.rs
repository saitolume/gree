use std::fs;
use std::path::PathBuf;

use failure::Error;
use regex::{Captures, Regex};

pub struct Branch {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: u32,
}

pub struct State {
    pub dir_count: u32,
    pub file_count: u32,
    pub branches: Vec<Branch>,
}

impl Branch {
    pub fn new(path: PathBuf) -> Branch {
        let is_dir = path.is_dir();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut depth: u32 = 0;

        let pattern = Regex::new(r"([^/]+/)").unwrap();
        for _ in pattern.captures_iter(&path.to_str().unwrap().to_string()) {
            depth += 1
        }

        Branch {
            name,
            path,
            is_dir,
            depth,
        }
    }

    pub fn println(&self, is_last: bool) {
        let path = &self.path.to_str().unwrap().to_string();
        let pattern = Regex::new(r"([^/]+/)").unwrap();
        let result =
            pattern.replace_all(path, |_: &Captures| if is_last { "└── " } else { "├── " });
        println!("{}", result);
    }

    pub fn child_node(&self, ignore_files: &Vec<String>) -> Result<State, Error> {
        let dir = fs::read_dir(&self.path)?;
        let mut branches: Vec<Branch> = vec![];
        let mut dir_count: u32 = 0;
        let mut file_count: u32 = 0;

        for entry in dir {
            let branch = Branch::new(entry?.path());
            if ignore_files.contains(&branch.name) {
                continue;
            }
            if branch.is_dir {
                let state = branch.child_node(ignore_files)?;
                branches.extend(state.branches);
            }
            if branch.is_dir {
                dir_count += 1
            } else {
                file_count += 1
            }
            branches.push(branch);
        }
        Ok(State {
            dir_count,
            file_count,
            branches,
        })
    }
}
