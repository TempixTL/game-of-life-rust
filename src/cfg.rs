use std::fs;

/// A struct representing the startup configuration for the applicaiton.
#[derive(Debug)]
pub struct Config {
    /// A string representation of the initial Game of Life board.
    pub starting_board: String,
    /// The number of iterations the `starting_board` should go through.
    pub steps: i32,
}

impl Config {

    /// Takes command-line arguments and parses them into a [Config].
    /// 
    /// # Arguments
    /// * `args` - Arguments from the command-line.
    /// 
    /// # Returns
    /// 
    /// A [Config] if successful, or a [ConfigError] with the cause of the
    /// failure.
    /// 
    /// ```
    /// use game_of_life::cfg::{Config, ConfigError};
    /// 
    /// # fn main() -> Result<(), ConfigError> {
    /// let args = [
    ///     "target/debug/game-of-life".to_string(),
    ///     "input/game-of-life-in1.txt".to_string(),
    ///     "3".to_string()
    /// ];
    /// 
    /// let config = Config::new(&args)?;
    /// assert_eq!(config.steps, 3);
    /// # Ok(())
    /// # }
    /// ```
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

/// An error which has occurred while creating a [Config].
#[derive(Debug)]
pub enum ConfigError {
    /// An error which occurs when the number of arguments passed to
    /// `Config::new` was incorrect. Contains both the `found` and `expected`
    /// number of arguments.
    ArgumentCountError { found: usize, expected: usize },
    /// An error which occurs when the specified file is not readable for some
    /// reason. Contains the `file_name` tried.
    FileAccessError { file_name: String },
    /// An error which occurs when the number of specified iterations of the
    /// board is invalid. Contains the number of iterations `found`, if a valid
    /// number at all.
    StepCountError { found: Option<i32> }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_fail_on_argument_count() {
        let args = [];
        match Config::new(&args) {
            Err(ConfigError::ArgumentCountError { found: 0, expected: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_fail_on_file_access() {
        let args = ["exe".to_string(), "/inaccessible_file".to_string(), "0".to_string()];
        match Config::new(&args) {
            Err(ConfigError::FileAccessError{ file_name: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_fail_on_step_count() {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "-1".to_string()];
        match Config::new(&args) {
            Err(ConfigError::StepCountError { found: _ }) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_succeed() {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "3".to_string()];
        assert!(Config::new(&args).is_ok());
    }

    #[test]
    fn config_new_should_return_correct_config() -> Result<(), ConfigError> {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "3".to_string()];
        let config = Config::new(&args)?;

        assert_ne!(config.starting_board.len(), 0);
        assert_eq!(config.steps, 3);
        Ok(())
    }
}