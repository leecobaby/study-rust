// 忽略测试操作
// ignore-test

// 通过 cargo test -- --ignored 来运行被忽略的测试
// cargo test --bin 11-8 -- --ignored

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn another() {
        assert_eq!(2 + 2, 4)
    }
}
