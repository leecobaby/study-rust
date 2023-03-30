// 消息传递 - Channel
// 一种很流行且能保证安全并发的技术
// 是使用消息传递来进行线程间的通信

// Channel 是一个通道，包含了一个发送端和一个接收端
// 创建 Channel：mpsc::channel() 它返回一个元组，包含了发送端和接收端

// try_recv() 方法不会阻塞，而是立即返回一个 Result<T, E> 值

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi");
        // 发送所有权
        tx.send(val).unwrap();
        // println!("val is {}", val);

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("1: more"),
            String::from("1: messages"),
            String::from("1: for"),
            String::from("1: you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    let received = rx.recv().unwrap(); // 阻塞主线程，直到接收到消息
    println!("Got: {}, only once", received);

    // 当 channel 关闭时，退出循环
    for received in rx {
        println!("Got: {}", received);
    }
}
