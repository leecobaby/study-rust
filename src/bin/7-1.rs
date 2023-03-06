// Package 和 Crate
// 一个包可以包含一个或多个库或二进制文件
// Crate 类型
// - binary crate
// - library crate

// Crate Root:
// - 是源代码文件
// - Rust 编译器从这个文件开始，组成一个 crate 的模块树

// Package
// - 一个 Cargo 工程可以包含一个或多个包
// - 一个包含有一个 Cargo.toml 文件的目录
// - 只能包含0-1个 library crate
// - 可以包含0-多个 binary crate
// - 但必须包含至少一个 crate（库或二进制文件）

// Cargo 惯例
// - src/main.rs 是一个 binary crate 的 crate root
// crate 名称是 package 名称

// - src/lib.rs 是一个 library crate 的 crate root
// crate 名称是 package 名称

// - src/main.rs 和 src/lib.rs 可以存在于同一个 package 中
// crate 名称是 package 名称

// - 一个 package 中可以包含多个 binary crate
// crate 名称是文件名
// 文件放在 src/bin 目录下
// 每个文件都是一个独立的 binary crate

// Module
// 在一个 crate 内，将代码进行分组
// 增加代码的可读性，易于复用
// 控制项目的私有性。(private, public)

// 建立 Module
// - mod 关键字
// 可嵌套
// 可包含其他项的定义：函数、结构体、枚举、常量、trait、impl 块
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main() {}

// src/main.rs 和 src/lib.rs 称为 crate root
// - 这两个文件（任意一个）的内容形成了名为 crate 的根模块，位于整个模块树的根
