// Drop Trait
// 实现 Drop Trait 使我们可以自定义当值离开作用域时的行为
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // 不允许手动调用 drop
    // c.drop(c);
    // 但可以调用 std::mem::drop 函数，它在预导入模块里
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
