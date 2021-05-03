use std::fs::File;

pub struct Config {
    pub file: File,
    pub steps: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ConfigException> {
        if args.len() != 3 {
            return Err(ConfigException::InvalidArgumentCount)
        }

        let file = File::open(args[1].clone());
        let steps = args[2].parse::<i32>();

        match (file, steps) {
            (Ok(file), Ok(steps)) if steps >= 0 => Ok(Config { file, steps }),
            (Err(_), _) => Err(ConfigException::FileAccess),
            (_, _) => Err(ConfigException::InvalidStepNumber),
        }
    }
}

pub enum ConfigException {
    InvalidArgumentCount,
    FileAccess,
    InvalidStepNumber,
}