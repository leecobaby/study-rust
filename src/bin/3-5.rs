fn main() {
    // if 表达式
    // if 表达式允许您根据条件来执行不同的代码分支
    // - 这个条件必须是 bool 类型
    // if 表达式可以返回值
    // if 表达式的返回值必须是相同的类型

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 表达式可以包含 else if 分支
    // else if 分支也必须是 bool 类型
    // else if 分支可以有多个
    // else 分支是可选的

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 表达式 赋值
    // if 表达式可以返回值，因此可以在 let 语句中使用它们
    let condition = true;

    // if 表达式的返回值必须是相同的类型
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
