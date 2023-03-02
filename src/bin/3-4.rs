// 函数
fn main() {
    println!("Hello, world!");
    another_function(5, 6); // argument
}

// 函数的参数
// parameter 是函数定义时的变量 形参
// argument 是函数调用时传递的值 实参
fn another_function(x: i32, y: i32) {
    // parameter
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // 语句和表达式
    // 语句是执行一些操作但不返回值的指令
    // 表达式计算并产生一个值
    // 语句以分号结尾，表达式不以分号结尾
    // 函数体由一系列语句和表达式组成，但是不以分号结尾
    // 函数体中的最后一个表达式的值会被隐式地返回

    // 函数的返回值
    // Rust 是一门表达式语言，函数的返回值是表达式的值
    // Rust 中的函数默认返回 unit 类型的值，即 ()
    // 如果想要返回其他类型的值，需要在函数签名中指定返回值类型
    // Rust 中的函数只能返回一个值，如果想要返回多个值，可以使用元组
    // Rust 中的函数可以返回一个指向堆上数据的指针，但是不能返回一个指向栈上数据的指针
    // Rust 中的函数可以返回一个引用，但是不能返回一个引用的引用
    // 可以使用 return 关键字提前返回函数

    let x = plus_five(6);
    println!("{}", x);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
