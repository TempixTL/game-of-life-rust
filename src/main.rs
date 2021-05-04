use std::env;
use game_of_life::cfg::{Config, ConfigError};
use game_of_life::engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => engine::run(config),
        Err(ConfigError::ArgumentCountError { found, expected }) =>
            eprintln!("Invalid number of arguments. Found {}, expected {}", found, expected),
        Err(ConfigError::StepCountError { found: Some(steps) }) =>
            eprintln!("Invalid step number. Found {}", steps),
        Err(ConfigError::StepCountError { found: None }) =>
            eprintln!("Invalid step number."),
        Err(ConfigError::FileAccessError { file_name }) =>
            eprintln!("Unable to open file '{}'.", file_name),
    }
}
