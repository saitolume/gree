use std::path::PathBuf;

pub struct Branch {
    pub name: String,
    pub is_dir: bool
}

impl Branch {
    pub fn new(path: PathBuf) -> Branch {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let is_dir = path.is_dir();
        if is_dir {
            return Branch {
                name: format!("{}/", name),
                is_dir
            };
        }
        Branch { name, is_dir }
    }

    pub fn println(&self, is_last: bool) {
        if is_last {
            println!("└── {}", &self.name);
        } else {
            println!("├── {}", &self.name);
        }
    }
}
