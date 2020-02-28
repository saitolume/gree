use std::path::Path;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        const DEFAULT_PATH: &str = ".";
        let mut path = DEFAULT_PATH.to_string();

        if args.len() > 2 {
            return Err("too enough arguments");
        }

        if args.len() == 2 {
            path = args[1].clone();
        }

        if !Path::new(&path).is_dir() {
            return Err("not a directory");
        }

        Ok(Config { path })
    }
}
