// 使用共享内存来实现并发
// mutex 互斥锁规则
// 在使用数据之前，必须尝试获取锁
// 使用完 mutex 之后，必须释放锁，以便其他线程可以获取锁
// Mutex<T> 是一个智能指针，它的 lock 方法返回一个叫做 MutexGuard 的智能指针
// 注意：Mutex<T> 有死锁风险

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *m.lock().unwrap());
}
