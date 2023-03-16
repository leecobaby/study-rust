// 实例：接收命令行参数
// 本例子中，我们将使用命令行参数来控制程序的行为。我们将使用一个命令行参数来指定要打印的行数，另一个命令行参数来指定要打印的文件名。如果没有指定文件名，我们将从标准输入读取内容。如果没有指定行数，我们将打印所有行。

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// 12-3 二进制程序分离指导性原则
// 将从程序拆分为 main.rs 和 lib.rs，将业务逻辑放入 lib.rs
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
