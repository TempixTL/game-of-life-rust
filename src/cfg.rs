use std::fs;
use crate::parser::{Parser, MassingillParser};

/// A struct representing the startup configuration for the applicaiton.
pub struct Config {
    /// A string representation of the initial Game of Life board.
    pub(crate) starting_board: String,
    /// The number of iterations the `starting_board` should go through.
    pub(crate) steps: i32,
    /// The parser to use when reading the `starting_board`.
    pub(crate) parser: Box<dyn Parser>,
}

impl Config {

    /// Takes command-line arguments and parses them into a [Config].
    /// 
    /// # Arguments
    /// * `args` - Arguments from the command-line.
    /// 
    /// # Returns
    /// 
    /// A [Result] of [Config] if successful, or a [String] with the cause of
    /// the failure.
    /// 
    /// ```
    /// use game_of_life::cfg::Config;
    /// 
    /// # fn main() -> Result<(), String> {
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
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(format!("Invalid number of arguments. Found {}, expected {}", args.len(), 3))
        }

        let starting_board = fs::read_to_string(args[1].clone()).map(|str| str.trim().to_string());
        let steps = args[2].parse::<i32>();

        match (starting_board, steps) {
            (Ok(starting_board), Ok(steps)) if steps >= 0 => Ok(Config { starting_board, steps, parser: Box::new(MassingillParser {}) }),
            (Err(_), _) => Err(format!("Unable to open file '{}'.", args[1])),
            (_, Ok(steps)) => Err(format!("Invalid step number. Found {}", steps)),
            (_, Err(_)) => Err("Invalid step number.".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_fail_on_argument_count() {
        let args = [];
        match Config::new(&args) {
            Err(err_str) if err_str.starts_with("Invalid number of arguments") => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_fail_on_file_access() {
        let args = ["exe".to_string(), "/inaccessible_file".to_string(), "0".to_string()];
        match Config::new(&args) {
            Err(err_str) if err_str.starts_with("Unable to open file") => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_fail_on_step_count() {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "-1".to_string()];
        match Config::new(&args) {
            Err(err_str) if err_str.starts_with("Invalid step number") => (),
            _ => panic!(),
        }
    }

    #[test]
    fn config_new_should_succeed() {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "3".to_string()];
        assert!(Config::new(&args).is_ok());
    }

    #[test]
    fn config_new_should_return_correct_config() -> Result<(), String> {
        let args = ["exe".to_string(), "input/game-of-life-in1.txt".to_string(), "3".to_string()];
        let config = Config::new(&args)?;

        assert_ne!(config.starting_board.len(), 0);
        assert_eq!(config.steps, 3);
        Ok(())
    }
}