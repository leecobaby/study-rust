mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use 作用：
// 1. 将路径引入作用域
// 2. 重构路径
use crate::front_of_house::hosting;
// use front_of_house::hosting; // 相对路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// use 惯用做法
// 函数使用 上一级目录
// 结构体，枚举，常量使用 结构体名::XX

// as 关键字
// 1. 重命名
// 2. 隐藏
use std::fmt::Result;
use std::io::Result as IoResult;
