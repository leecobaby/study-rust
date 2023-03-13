// 编写和运行测试
// 1. 编写测试
// 2. 运行测试
//  - 使用 cargo test 运行测试

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

fn main() {
    println!("Hello, world!");
}

// 测试失败
// 测试触发panic，就表示测试失败
