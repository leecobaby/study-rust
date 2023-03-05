// Option 枚举
// 定义于标准库中的 Option 枚举是一个很有用的工具，因为它可以用来表示可能存在或可能不存在的值。
// 在 Prelude (预导入模块)中。

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
