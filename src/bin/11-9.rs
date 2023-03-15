// 测试的分类
// 1. 单元测试
// 小规模的测试，测试单个函数
// 可以测试 private 函数
// 2. 集成测试
// 集成测试，测试多个函数
// 集成测试只能测试 public 函数
// 在😭外部

// #[cfg(test)]标准
// 通过 cargo test 才会编译和运行测试
// 运行 cargo build 时不会编译测试

// cfg: configuration
// 告诉 Rust 编译器，只有在特定的配置下才编译某些代码

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
