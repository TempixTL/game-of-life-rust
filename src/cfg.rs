use std::fs;

pub struct Config {
    pub starting_board: String,
    pub steps: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ConfigError> {
        if args.len() != 3 {
            return Err(ConfigError::ArgumentCountError { found: args.len(), expected: 3 })
        }

        let starting_board = fs::read_to_string(args[1].clone()).map(|str| str.trim().to_string());
        let steps = args[2].parse::<i32>();

        match (starting_board, steps) {
            (Ok(starting_board), Ok(steps)) if steps >= 0 => Ok(Config { starting_board, steps }),
            (Err(_), _) => Err(ConfigError::FileAccessError { file_name: args[1].clone() }),
            (_, Ok(steps)) => Err(ConfigError::StepCountError { found: Some(steps) }),
            (_, Err(_)) => Err(ConfigError::StepCountError {found: None }),
        }
    }
}

pub enum ConfigError {
    ArgumentCountError { found: usize, expected: usize },
    FileAccessError { file_name: String },
    StepCountError { found: Option<i32> }
}