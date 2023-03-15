// 集成测试
// 在 Rust 里，集成测试完全位于被测试库的外部
// 目的：测试被测试库的多个部分是否正确的一起工作

// tests 目录
// tests 目录下的每个测试文件都会被编译为一个独立的 crate
// 运行特定测试文件的测试：cargo test --test <test_file_name>
// 运行特定测试函数的测试：cargo test <test_function_name>

// 针对 binary crate 的集成测试
// 如果项目是 binary crate，那么不能在 tests 目录下创建测试文件，无法把 main.rs 的函数导入作用域
// 只有 libary crate 才能暴露函数给其他 crate 使用
// binary crate 意味着独立运行
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
