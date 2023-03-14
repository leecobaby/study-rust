// 控制测试如何运行
// 默认行为
//  - 并行运行
//  - 所有测试
//  - 捕获（不显示）所有输出

// 通过命令行参数控制测试
//  - cargo test --help
//  - 并行测试，确保测试之间不会相互依赖
//  - 串行测试，cargo test -- --test-threads=1

// 测试成功，不会显示 println! 输出
// 如果想在测试成功时显示输出，可以使用 --show-output 参数
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
