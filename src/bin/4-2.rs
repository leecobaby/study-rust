fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    // 引用和借用
    // 引用允许您引用某些值，而不获取其所有权
    // 符号 & 表示引用

    let r = dangle();
}

fn calculate_length(s: &mut String) -> usize {
    // 引用作为函数参数称为借用
    // 借用是不可变的，因为函数不会更改其值
    // 如果要在函数中更改引用的值，必须使用可变引用
    // 一段代码的任何给定时间只能拥有对某个数据的不可变引用或可变引用，但不能同时拥有这两种引用
    s.len()
}

// 一下三种行为下发生数据竞争：
// 1. 两个或更多指针同时访问同一数据
// 2. 至少有一个指针被用于写入数据
// 3. 没有同步数据访问的机制

// 悬空引用
// 悬空引用是指在某个引用的值可能已经被释放的情况下引用该值的情况
// Rust 编译器确保引用永远不会成为悬空引用
fn dangle() -> &String { // 编译器会报错
    let s = String::from("hello");
    &s
}
