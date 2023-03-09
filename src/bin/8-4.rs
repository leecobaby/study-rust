fn main() {
    let s1 = String::from("hello");
    // let h = s1[0]; // error
    // String 是一个 Vec<u8> 的包装类型

    let len = String::from("Hola").len();
    // Unicode 标量值的范围从 U+0000 到 U+D7FF 以及 U+E000 到 U+10FFFF
    println!("{}", len); // 4

    // Rust 有三种看待字符串的方式：
    //  - 字节序列（byte sequence）
    //  - Unicode 标量值（Unicode scalar values）
    //  - 字符（grapheme clusters）

    // 字节序列
    //  - 字节序列是一系列的 8 位值（u8）
    // 梵文
    let w = String::from("नमस्ते");

    for b in w.bytes() {
        println!("{}", b);
    }

    // Unicode 标量值
    //  - Unicode 标量值是一系列的 Unicode 码位（code point）
    for c in w.chars() {
        println!("{}", c);
    }

    // 字符
    //  - 字符是一个 Unicode 标量值的集合
    //  - 一个字符可能由一个或多个 Unicode 标量值组成
    //  - 一个字符可能占用一个或多个字节
    // 标准库里没提供直接访问字符的方法

    // Rust 不允许对 String 进行索引操作的原因
    //  - 索引操作的时间复杂度是 O(1)
    //  - 但是对于 String 来说，索引操作的时间复杂度是 O(n)

    // 切割 String
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("{}", s);
    // 注意：切割 String 的索引必须位于字符边界上，否则会 panic
    
}
