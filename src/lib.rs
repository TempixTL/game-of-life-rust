pub mod cfg {
    use std::fs::File;

    pub struct Config {
        pub file: File,
        pub steps: i32,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, Exception> {
            if args.len() != 3 {
                return Err(Exception::InvalidArgumentCount)
            }

            let file = File::open(args[1].clone());
            let steps = args[2].parse::<i32>();

            match (file, steps) {
                (Ok(file), Ok(steps)) if steps >= 0 => Ok(Config { file, steps }),
                (Err(_), _) => Err(Exception::FileAccess),
                (_, _) => Err(Exception::InvalidStepNumber),
            }
        }
    }

    pub enum Exception {
        InvalidArgumentCount,
        FileAccess,
        InvalidStepNumber,
    }
}