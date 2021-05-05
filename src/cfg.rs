use std::fs;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ConfigError {
    ArgumentCountError { found: usize, expected: usize },
    FileAccessError { file_name: String },
    StepCountError { found: Option<i32> }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_should_argument_count_error() {
        let args = [];
        match Config::new(&args) {
            Err(ConfigError::ArgumentCountError { found: 0, expected: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_should_file_access_error() {
        let args = ["exe".to_string(), "/inaccessible_file".to_string(), "0".to_string()];
        match Config::new(&args) {
            Err(ConfigError::FileAccessError{ file_name: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_should_step_count_error() {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "-1".to_string()];
        match Config::new(&args) {
            Err(ConfigError::StepCountError { found: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_should_return_config() -> Result<(), ConfigError> {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "3".to_string()];
        let config = Config::new(&args)?;

        assert_ne!(config.starting_board.len(), 0);
        assert_eq!(config.steps, 3);
        Ok(())
    }
}