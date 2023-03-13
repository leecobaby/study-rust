// 生命周期标注语法
// 通过在函数签名中使用生命周期标注语法来指定每个引用的生命周期
// 生命周期的标准不会改变任何引用的生命周期

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", result);
}

// 悬垂引用：指向已经被丢弃的值的引用
// 如果想让函数里的变量在函数外也能使用，就需要把所有权转移出去
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
