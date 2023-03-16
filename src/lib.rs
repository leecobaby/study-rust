use std::{error::Error, fs};

// 在不同的场景下返回不同的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
    // ? 运算符会返回错误值给调用者
        fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

// 12-3 二进制程序分离指导性原则
// 将从程序拆分为 main.rs 和 lib.rs，将业务逻辑放入 lib.rs
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
