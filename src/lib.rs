use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for query: '{}'\nin path: '{}'", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("File with text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>   {
        if args.len() < 3 {
            return Err("not enough arguments provided")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
