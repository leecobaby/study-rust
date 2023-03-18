// 实例：接收命令行参数
// 本例子中，我们将使用命令行参数来控制程序的行为。我们将使用一个命令行参数来指定要打印的行数，另一个命令行参数来指定要打印的文件名。如果没有指定文件名，我们将从标准输入读取内容。如果没有指定行数，我们将打印所有行。

use std::env;
use std::process;
// 导入 ./src/bin/12-lib.rs 其他的 crate
use study_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 模式匹配用法
    if let Err(e) = study_rust::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
