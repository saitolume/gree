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

impl Branch {
    pub fn new(path: PathBuf) -> Result<Branch, Error> {
        let is_dir = path.is_dir();
        let mut depth: u32 = 0;
        let pattern = Regex::new(r"([^/]+/)")?;
        for _ in pattern.captures_iter(&path.to_str().unwrap().to_string()) {
            depth += 1
        }

        Ok(Branch {
            name: if path.file_name().is_some() {
                path.file_name().unwrap().to_str().unwrap().to_string()
            } else {
                path.to_str().unwrap().to_string()
            },
            path,
            is_dir,
            depth,
        })
    }

    pub fn row(&self, is_last: bool) -> String {
        let path = self.path.to_str().unwrap().to_string();
        let pattern = Regex::new(r"([^/]+/)").unwrap();
        pattern.replace_all(&path, |_: &Captures| if is_last { "└── " } else { "├── " }).to_string()
    }

    pub fn read_children(
        &self,
        ignore_files: &Vec<String>,
    ) -> Result<(std::vec::Vec<Branch>, u32, u32), Error> {
        let dir = fs::read_dir(&self.path)?;
        let mut branches: Vec<Branch> = vec![];
        let mut dir_count: u32 = 0;
        let mut file_count: u32 = 0;

        for entry in dir {
            let path = entry?.path();
            if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
                continue;
            }

            let branch = Branch::new(path)?;
            if ignore_files.contains(&branch.name) {
                continue;
            }

            if branch.is_dir {
                let (children, c_dir_count, c_file_count) = branch.read_children(ignore_files)?;
                dir_count += c_dir_count + 1;
                file_count += c_file_count;
                branches.extend(children);
            } else {
                file_count += 1;
            }
            branches.push(branch);
        }

        Ok((branches, dir_count, file_count))
    }
}
