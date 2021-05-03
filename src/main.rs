use std::env;
use game_of_life::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => println!("{:?}", config.steps),
        Err(ConfigException::InvalidArgumentCount) =>
            eprintln!("Invalid number of arguments."),
        Err(ConfigException::InvalidStepNumber) =>
            eprintln!("Invalid step number."),
        Err(ConfigException::FileAccess) =>
            eprintln!("Unable to open file."),
    }
}
