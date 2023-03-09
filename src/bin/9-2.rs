// Result 枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    // let a = match &f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // let b = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("There was a problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // 匹配不同的错误
    // match 很有用，但是有时候太长了，可以使用 if let 语法
    // 闭包方式，Result 有很多方法。

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    // unwrap 和 expect
    // unwrap 是一个很方便的方法，如果 Result 是 Ok 值，unwrap 会返回 Ok 中的值。
    // 如果 Result 是 Err 值，unwrap 会调用 panic! 宏。
    // expect 也是一个很方便的方法，如果 Result 是 Ok 值，expect 会返回 Ok 中的值。
    // 如果 Result 是 Err 值，expect 会调用 panic! 宏，并且会把 Err 中的值作为 panic! 宏的参数。
    // 一般来说，expect 比 unwrap 更有用，因为它可以提供更多的信息。
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
