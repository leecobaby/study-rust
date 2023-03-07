mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 外部访问 hosting
pub use crate::front_of_house::hosting;

// 使用外部包 package
// 1. cargo.toml 中添加依赖包 （package）
// 2. use 将特定条目引入作用域

use rand::Rng;
use std::collections::HashMap;

// 使用嵌套路径来清理大量使用
use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;
use std::io::{self, Write};
// use std::id::*;
