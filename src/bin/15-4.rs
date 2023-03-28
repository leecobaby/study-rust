// Rc<T> 引用计数智能指针，reference counting smart pointer
// 有时，一个值会有多个所有者
// 追踪所有到值的已用，当 0 个引用，值将被丢弃
// 它只能应用于单线程场景
// 他只能共享不可变引用，不能共享可变引用
// 它不在预导入模块里面，需要手动导入
// Rc::clone(&a) 会增加 Rc<T> 的引用计数
// Rc::strong_count(&a) 会返回 Rc<T> 的引用计数

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Rc::clone() 会增加引用，不会深拷贝
// 类型的 clone 方法会深拷贝
