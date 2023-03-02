// 常量
// 常量是不可变的，而且常量只能被设置为常量表达式，不能设置为函数的返回值
// 常量的命名习惯是全部大写，用下划线分隔单词
// 常量可以在任何作用域中声明，包括全局作用域
// 常量必须声明类型
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    // Shadowing （隐藏）
    // 可以使用相同的变量名，不过使用let关键字，这样可以改变变量的类型
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing 和 mut 的区别
    // mut 可以改变变量的值，但是类型不能改变
    // Shadowing 可以改变变量的类型，但是值不能改变
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
