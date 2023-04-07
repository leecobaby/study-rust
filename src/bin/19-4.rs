// 函数指针
// 可以将函数传递给其他函数，这样就可以在不同的上下文中执行相同的代码。
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    enum Status {
        Value(u32),
        Stop,
    }

    // 元祖结构体，也实现了 Fn trait
    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// 函数指针与闭包不同
// fn 是一个类型，不是一个 trait

// 返回闭包的函数
// 闭包使用 trait 来表达，无法在函数中直接返回闭包，可以将一个实现了 trait 的具体类型作为返回值

// 因为返回的是函数指针指向的函数地址，所以编译器无法确定返回的内存大小，所以报错
// fn retruns_clousure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// 把函数地址放在一个指针后面，就可以返回一个大小能确定的指针了
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
