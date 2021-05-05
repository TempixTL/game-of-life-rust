use std::env;
use game_of_life::cfg::{Config, ConfigError};
use game_of_life::engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => engine::run(config),
        Err(ConfigError::ArgumentCountError { found, expected }) => {
            eprintln!("Invalid number of arguments. Found {}, expected {}", found, expected);
            print_help(&args[0]);
        },
        Err(ConfigError::StepCountError { found: Some(steps) }) => {
            eprintln!("Invalid step number. Found {}", steps);
            print_help(&args[0]);
        },
        Err(ConfigError::StepCountError { found: None }) => {
            eprintln!("Invalid step number.");
            print_help(&args[0]);
        },
        Err(ConfigError::FileAccessError { file_name }) => {
            eprintln!("Unable to open file '{}'.", file_name);
            print_help(&args[0]);
        },
    }
}

fn print_help(prog_name: &str) {
    eprintln!("\
Simulates Conway's Game of Life.

Syntax: {} INIT_BOARD_FILE STEP_NUMBER
Options:
INIT_BOARD_FILE - Path to the initial Game of Life board.
STEP_NUMBER - Number of steps to iterate. Should be a positive number.",
    prog_name);
}