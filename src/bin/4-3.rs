fn main() {
    let mut s = String::from("hello world");
    let word_index = first_word(&s);
    let word_slice = first_word_slice(&s);

    s.clear(); // 这清空了字符串，使其等于 ""
    println!("The first word is: {}", word_slice);

    let s1 = String::from("hello world");
    // let hello = &s1[0..5];
    let hello = &s1[..5];
    // let world = &s1[6..11];
    let world = &s1[6..];
    let whole = &s1[..];
    println!("{}, {}", hello, world);

    let s2 = String::from("hello world");
    let frist_slice = first_word_slice(&s2[..]);
    println!("{}", frist_slice);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 字符串切片
// 字符串切片是对字符串中一部分内容的引用
// 字符串切片的类型是 &str
// 形式：[start..end]
// start 是字符串中第一个字节的索引
// end 是字符串中最后一个字节的索引加 1
// fn first_word_slice(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 字符串字面值是字符串切片
// 字符串字面值是字符串切片的简写
// 字符串字面值是不可变的
// 字符串字面值是静态分配的，它们存储在二进制文件中的只读部分

// 将字符串切片作为参数传递，可以同时接受 String 和 &str 类型的值，并保证借用的值不可变
// fn first_word_slice(s: &str) -> &str {
