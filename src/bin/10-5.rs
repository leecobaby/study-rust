fn main() {
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }

    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 借用检察器
// Rust 在编译时检查借用的有效性
// 保证内存安全
// 保证数据不会在其引用的有效期

// 一旦返回了引用，被引用的值就不能被丢弃，所以需要申明生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
