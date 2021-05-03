use std::env;
use game_of_life::cfg::Config;
use game_of_life::cfg;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => println!("{:?}", config.steps),
        Err(cfg::ConfigError::ArgumentCountError { found, expected }) =>
            eprintln!("Invalid number of arguments. Found {}, expected {}", found, expected),
        Err(cfg::ConfigError::StepCountError { found: Some(steps) }) =>
            eprintln!("Invalid step number. Found {}", steps),
        Err(cfg::ConfigError::StepCountError { found: None }) =>
            eprintln!("Invalid step number."),
        Err(cfg::ConfigError::FileAccessError { file_name }) =>
            eprintln!("Unable to open file '{}'.", file_name),
    }
}
