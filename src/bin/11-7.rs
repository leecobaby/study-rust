// 按名称运行测试
// 通过 cargo test test_name 来运行指定的测试
// 可以这么使用 `cargo test --bin <binary_file_name> <test_function_name>`

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
