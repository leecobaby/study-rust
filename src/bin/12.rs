// 实例：接收命令行参数
// 本例子中，我们将使用命令行参数来控制程序的行为。我们将使用一个命令行参数来指定要打印的行数，另一个命令行参数来指定要打印的文件名。如果没有指定文件名，我们将从标准输入读取内容。如果没有指定行数，我们将打印所有行。

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
