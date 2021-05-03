pub mod cfg {
    use std::fs::File;

    pub struct Config {
        pub file: File,
        pub steps: i32,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, ConfigError> {
            if args.len() != 3 {
                return Err(ConfigError::ArgumentCountError { found: args.len(), expected: 3 })
            }

            let file = File::open(args[1].clone());
            let steps = args[2].parse::<i32>();

            match (file, steps) {
                (Ok(file), Ok(steps)) if steps >= 0 => Ok(Config { file, steps }),
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
}