use std::env;
use game_of_life::cfg::Config;
use game_of_life::cfg;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => println!("{:?}", config.steps),
        Err(cfg::Exception::InvalidArgumentCount) =>
            eprintln!("Invalid number of arguments."),
        Err(cfg::Exception::InvalidStepNumber) =>
            eprintln!("Invalid step number."),
        Err(cfg::Exception::FileAccess) =>
            eprintln!("Unable to open file."),
    }
}
