fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // 内存和分配
    // 字符串字面值，在编译时就已知其大小，因此文本被硬编码到最终的可执行文件中
    // - 速度快，高效。是因为其不可变性。
    // String 类型，其大小在运行时是未知的，因为它需要在运行时才能确定要分配多少内存来存储其内容

    // Rust 采用了不同的方式：对于某个值老说，当拥有他的变量走出作用域时，内存会立即自动的交换给操作系统
    // drop 函数会在变量离开作用域时自动调用

    // 变量和数据交互的方式：移动（Move）
    // 多个变量可以与同一个数据使用一种独特的方式交互
    // 因为他们在 stack 上所以这个过程会变成 copy 操作
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // 因为他在 heap 上所以这个过程会变成 move 操作
    // let s2 = s1;
    // println!("{}, world!", s1); // error[E0382]: borrow of moved value: `s1`
    let s3 = s1.clone();
    println!("{}, world!", s3); // hello, world!

    // 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
    // 所有整数类型，比如 u32
    // 布尔类型，bool，它的值是 true 和 false
    // 所有浮点数类型，f64 和 f32
    // 字符类型，char
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是

    // 所有权和函数
    // 函数的参数也是变量，因此也遵循相同的模式
    // - 将值传递给函数时，将值的数据移动（或复制）到函数的参数中
    // - 当函数结束时，参数将被丢弃，就像它们被丢弃一样
    takes_ownership(s3);

    makes_copy(y);

    println!("{}", y);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
    // Here, some_integer goes out of scope. Nothing special happens.
}
