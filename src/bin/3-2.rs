fn main() {
    // 基于使用的值，编译器通常可以推断出具体类型
    // 但是有时候需要明确指定类型
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {}", guess);

    // 标量类型
    // 一个标量类型代表一个单独的值
    // Rust 有四种基本的标量类型：整型、浮点型、布尔型和字符型
    // 整型
    // 有符号整型：i8、i16、i32、i64、i128 和 isize（指针大小）
    // 无符号整型：u8、u16、u32、u64、u128 和 usize（指针大小）
    // isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的
    // Rust 的默认整型是 i32：这通常是最快的，即使在今天的 64 位系统上
    // Rust 的默认浮点型是 f64：这也是科学计算中默认的类型
    // 数字字面值
    // 整数字面值
    // 十进制：98_222
    // 十六进制：0xff
    // 八进制：0o77
    // 二进制：0b1111_0000
    // 数字字面值可以使用下划线增加可读性，但是不能在数字中间加下划线
    // 浮点数字面值
    // 默认是 f64 类型
    // 后缀 f32 表示 f32 类型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {} {}", x, y);

    // 数值运算
    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 取余
    let remainder = 43 % 5;

    // 布尔类型
    // Rust 中的布尔类型是 bool，它的值只能是 true 或 false
    // 占用一个字节
    let bool = true;

    // 字符类型
    // Rust 的 char 类型的大小是四个字节，代表一个 Unicode 标量值
    // Unicode 标量值在范围 \u{0} 到 \u{D7FF} 或 \u{E000} 到 \u{10FFFF} 之间的任意字符
    let str1 = 'z';
    let str2 = 'ℤ';
    let str3 = '😂';
}
