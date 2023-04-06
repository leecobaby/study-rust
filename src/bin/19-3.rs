// 高级类型

// 类型别名
type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

use std::fmt;

// 为 Result<T, E> 定义一个别名
type Result<T> = std::io::Result<T>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// Never Type ! 可以强制的转换为其他任意类型
fn bar() -> ! {
    // --snip--
}

// str 是动态大小类型，所以需要使用 &str，这样他只在栈上存放str的地址和长度
// 动态类型都是在堆上存放指针
// 每个 trait 都是一个动态大小的类型，可以通过名称对其进行引用
// 为了将 trait 用作 trait 对象，必须将它放置在某个指针之后，如 Box<T> 或 &T

fn generic<T>(t: T) {
    // --snip--
}

// 编译已知大小的类型
fn generic<T: Sized>(t: T) {
    // --snip--
}

// 编译未知大小的类型
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
