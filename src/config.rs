pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = std::env::var("CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            case_sensitive
        })
    }
}
