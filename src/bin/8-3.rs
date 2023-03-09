// String 类型
//  - 来自标准库的 String 类型是一个由 UTF-8 编码的可增长的文本块
//  - String 类型是一个由 Vec<u8> 实现的包装类型
fn main() {
    // 函数
    let mut s = String::new();
    let data = "initial contents";
    // 方法
    let s = data.to_string();
    let s = "initial contents".to_string();
    // 函数
    let s = String::from("initial contents");
    // 通过 push_str 方法将一个字符串切片附加到 String
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('l');
    println!("{}", s);

    // 通过 + 运算符或 format! 宏来连接多个字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 使用了类似这个签名的方法 fn add(self, s: &str) -> String {}
    // &String 被解引用强制转换 (deref coercion)
    let s3 = s1 + &s2; // s1 被移动了，不能再使用
    println!("{}", s3);
    // println!("{}", s1); // error
    println!("{}", s2);
    let s4 = format!("{}-{}", s2, s3);
    println!("{}", s4);
}
