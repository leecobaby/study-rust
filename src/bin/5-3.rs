// struct 方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// impl 块
// impl 块允许你为特定的类型实现外部 trait，或者为无法修改源码的类型实现 trait。
// impl 块是在类型的上下文中实现 trait 的另一种方式。
// 可以在 impl 块里定义方法
// 方法的第一个参数总是 self，它代表调用该方法的实例，可以是可变的（&mut self）或不可变的（&self）
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    // 关联函数不是结构体的方法
    // 关联函数不需要 self 参数
    // 关联函数通常用作构造函数
    // 有点类似于静态方法
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// struct 可以有多个 impl 块
impl Rectangle {}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("{} {}", rect1.can_hold(&rect2), rect1.can_hold(&rect3));

    // 关联函数调用语法
    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);
}
