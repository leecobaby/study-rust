// crate.io
// 可以通过发布包来共享自己的代码

// 文档注释
// 通过 /// 开头的注释，可以生成文档
// 支持 Markdown 语法
// 放置在被说明条目之前
// 通过 cargo doc --open 可以生成文档并打开
// cargo doc --open --bin 14-2 生成并打开此二进制文件的文档

// 其他常用文档注释
// - Panics: 说明函数可能会 panic 的情况
// - Errors: 说明函数可能会返回错误的情况
// - Safety: 说明函数调用时需要满足的安全性要求

// 文档测试
// 示例代码块 可以通过 cargo test --doc 来测试

// //! 为一个 crate 添加文档注释

//! # 14-2 create
//!
//! `study_rust` is a collection of utilities to make performing certain...

/// Adds one to the number given.
///
/// # Examples
///   
/// ```
/// let arg = 5;
/// let answer = study_rust::add_one(arg);
///   
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
