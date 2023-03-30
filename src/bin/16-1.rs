// 并发
// 多线程同时运行代码
// Rust 标准库提供 1:1 线程模型
// thread::spawn 函数创建新线程，接受一个闭包作为参数，返回一个 JoinHandle 实例
// JoinHandle 实例代表了一个新线程，持有值的所有权，可以调用 join 方法来等待线程结束

// 使用 move 闭包，可以在创建线程时，把所有权转移给新线程

use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
