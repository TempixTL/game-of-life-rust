use std::env;
use game_of_life::cfg::{Config, ConfigError};
use game_of_life::engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => engine::run(config),
        Err(ConfigError::ArgumentCountError { found, expected }) => {
            print_help(
                &format!("Invalid number of arguments. Found {}, expected {}", found, expected),
                &args[0]);
        },
        Err(ConfigError::StepCountError { found: Some(steps) }) => {
            print_help(
                &format!("Invalid step number. Found {}", steps),
                &args[0]);
        },
        Err(ConfigError::StepCountError { found: None }) => {
            print_help(
                &format!("Invalid step number."),
                &args[0]);
        },
        Err(ConfigError::FileAccessError { file_name }) => {
            print_help(
                &format!("Unable to open file '{}'.", file_name),
                &args[0]);
        },
    }
}

fn print_help(err_msg: &str, prog_name: &str) {
    eprintln!("{}
Simulates Conway's Game of Life.

Syntax: {} INIT_BOARD_FILE STEP_NUMBER
Parameters:
INIT_BOARD_FILE - Path to the initial Game of Life board.
STEP_NUMBER - Number of steps to iterate. Should be a positive number.",
    err_msg, prog_name);
}