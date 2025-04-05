use std::fs;
use std::error::Error;

pub mod config;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let config = config::Config::new(&args)?;
    let content = fs::read_to_string(&config.file_path)?;
    // println!("Searching for `{}` in `{}`",config.query, config.file_path);
    // println!("File content:\n{content}");
    let result = search(&config.query, &content);
    for m in result {
        println!("{m}");
    }
    Ok(())
}

pub fn search<'a>( query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests;
