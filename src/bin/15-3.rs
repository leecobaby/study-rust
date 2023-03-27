// 函数和方法的隐式解引用转化 Deref coercion
// 假设 T 实现了 Deref trait，那么 &T 将会自动转化为 &U，其中 U 是 T 实现 Deref trait 返回的类型的引用。
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // &m => &MyBox<String> => deref(&String) => &String => deref(&str) => &str
    hello(&m);
    // hello(&(*m)[..]);
}

// 解引用和可变性
// 可以使用 DerefMut trait 重载可变引用的 * 运算符
// 当 T: Deref<Target=U> 时，&T 可以被转化为 &U
// 当 T: DerefMut<Target=U> 时，&mut T 可以被转化为 &mut U
// 当 T: Deref<Target=U> 时，&mut T 可以被转化为 &U
