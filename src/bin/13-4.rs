// 闭包可以捕获他们所在的环境
// 闭包可以访问定义它的作用域内的变量，而函数不能
// 会产生内存开销
// FnOnce - 获取所有权且被调用一次，所有的闭包都是 FnOnce 的子 trait
// FnMut - 可变借用，没有移动捕获变量会实现这个 trait
// Fn - 不可变借用

// move 关键字
// 在参数列表前使用 move 关键字，强制闭包获取其使用的值的所有权
// - 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最有用

fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
