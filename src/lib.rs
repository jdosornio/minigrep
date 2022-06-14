use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Need at least 2 parameters");
        }

        Ok(Self {
            filename: args[2].to_owned(),
            query: args[1].to_owned(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("Failed to read file");

    println!("With text: \n{}", contents);

    Ok(())
}
