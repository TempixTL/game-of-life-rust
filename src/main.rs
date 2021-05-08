use std::env;
use game_of_life::cfg::Config;
use game_of_life::engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => if let Err(error) = engine::run(config) {
            eprintln!("Error while running: {}", error);
        },
        Err(err_str) => print_help(&err_str, &args[0]),
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