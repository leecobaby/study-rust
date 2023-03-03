// 循环
fn main() {
    let mut counter = 0;

    // loop 表达式
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while 表达式
    // while 表达式允许您在循环中执行代码，直到某个条件为 false 为止
    // while 表达式可以返回值
    // while 表达式的返回值必须是相同的类型
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for 表达式
    // for 表达式允许您在循环中执行代码，直到某个条件为 false 为止
    // for 表达式可以返回值
    // for 表达式的返回值必须是相同的类型
    // 使用 for 循环更简洁紧凑，它可以针对集合中的每个元素执行代码
    let a = [10, 20, 30, 40, 50];

    // for 更安全，因为它在执行时不会访问超出数组边界的元素
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Range
    // 标准库提供了一个 Range 类型，它可以创建一个范围，包括起始值但不包括结束值
    // rev 方法可以反转 Range，从而创建一个倒序的 Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
