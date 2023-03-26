// Box<T> 是最简单的智能指针
// 它允许在堆上分配值，而不是在栈上分配值
// stack 上是指向 heap 的指针

// Box<T> 常用场景
// 1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// 2. 当有大量数据并希望将其所有权转移给函数而不是只是借用的时候
// 3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

// 使用 Box 来获得确定大小的递归类型
// 因为他是一个指针，指针大小是固定
use crate::List::{Cons, Nil};
fn main() {
    // 1. 在编译时未知大小的类型
    // 2. 大量数据并希望将其所有权转移给函数
    let b = Box::new(5);
    println!("b = {}", b);

    // 3. 只关心它的类型是否实现了特定 trait
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);
}

// Cons List
// Const List 是来自 Lisp 的一种数据结构
// Const 里面的成员由两个元素组成
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
