// 适合 panic！的列子
// 演示某些概念：unwrap
// 原型代码：unwrap expect
// 测试
// 你可以确定 Result 是 OK：unwrap
use std::net::IpAddr;
fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // ...

    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let guess = Guess::new(guess);
    let v = guess.value();
    println!("v: {}", v);
    println!("guess: {}", guess.value);
    // ...
}

// 场景建议
// 调用代码，传入无意义的参数：panic!
// 调用外部不可控代码，返回非法状态，无法处理：panic!
// 如果失败是可预期的：返回 Result

// 为验证创建自定义类型
// 创建新的类型，把验证逻辑放在构造实例里的函数里

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
