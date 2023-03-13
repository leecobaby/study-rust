fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 悬垂引用：指向已经被丢弃的值的引用
// 如果想让函数里的变量在函数外也能使用，就需要把所有权转移出去
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// 结构体内定义引用类型
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期的省略规则
// 1. 每一个是引用的参数都有它自己的生命周期参数
// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
// 3. 如果方法有多个输入生命周期参数，但其中之一是 &self 或 &mut self，那么 self 的生命周期被赋予所有输出生命周期参数
