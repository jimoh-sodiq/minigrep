use std::{error::Error, fs};
pub struct Config {
    file_path: String,
    query: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided from the command line");
        }
        Ok(Config {
            file_path: args[1].clone(),
            query: args[2].clone(),
        })
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path())?;
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(config.query()) {
            println!("{}", line);
            results.push(line);
        }
    }

    println!("{:?}", results);
    Ok(())
}
