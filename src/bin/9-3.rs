// 传播错误
// 将错误传播给调用者
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // ? 与 from 函数
    // 当 ? 所应用的错误，会隐式的被 from 函数处理
    // 用于针对不同的错误原因，返回同一种错误类型

    // 链式调用
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = read_username_from_file();
    println!("{:?}", f);
    let f = read_username_from_file2();
    println!("{:?}", f);
    let f = File::open("hello.txt")?;
    Ok(())
}

// ? 运算符与 main 函数
// main 函数的返回类型是 ()
// 当 main 函数返回类型也可以是 Result<T, E> 时，可以使用 ? 运算符
