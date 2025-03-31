use std::fs;
use std::error::Error;

pub mod config;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let config = config::Config::new(&args)?;
    let content = fs::read_to_string(&config.file_path)?;
    println!("Searching for `{}` in `{}`",config.query, config.file_path);
    println!("File content:\n{content}");
    Ok(())
}

#[cfg(test)]
mod tests;
