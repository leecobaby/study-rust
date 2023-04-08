// 宏 macro
// 1. 宏定义
// 宏在 rust 里指的是一组相关特性的集合称呼
// 使用 macro_rules! 声明宏
// 函数与宏的差别
// 本质上，宏是用来编写可以生成其他代码的代码
// 函数在定义签名时，必须声明参数的额个数和类型，宏可处理可变的参数
// 编译器会在解释代码时展开宏，而不是在编译时
// 宏的定义比函数复杂的多，难以阅读，理解，维护

// let v = vec![1, 2, 3, 4, 5];

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// rust 宏的编写 和 AST方面比较薄弱目前使用为主，以后继续深入研究
