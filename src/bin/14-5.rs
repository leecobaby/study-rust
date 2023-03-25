// Cargo 工作空间 （workspances）
// 工作空间：帮助管理多个相互关联且需要协同开发的 crate
// 工作空间共享同一个 Cargo.lock 和输出文件夹包

// 列如一个空间 1个二进制crate，2个库crate
// 二进制crate：main 函数，依赖于其它2个库crate
// 其中1个库crate提供add_one函数
// 另一个库crate提供add_two函数

// cargo new adder
// cargo new add-one --lib
// cargo new add-two --lib
// cargo run -p adder
// cargo test -p add-one
// Cargo.toml
// [workspace]

// members = [
//  "adder"
//  "add-one"
// ]


// ./adder/Cargo.toml
// [dependencies]
// add-one = { path="../add-one" }