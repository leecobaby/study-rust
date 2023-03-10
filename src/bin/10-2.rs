// 泛型
// 提高代码复用能力
// 例如：fn largest<T>(list: &[T]) -> T {...}
// 类型参数
//  - 很短，通常一个字母
//  - CamelCase
//  - T: type 缩写

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = Po { x: 5, y: 10 };
    let q = Po { x: "Hello", y: 'c' };
    let r = p.mixup(q);
    println!("r.x = {}, r.y = {}", r.x, r.y);
}

struct Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}

// 方法中定义泛型：表示在类型 T 上实现的方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Po<T, U> {
    x: T,
    y: U,
}

impl<T, U> Po<T, U> {
    fn mixup<V, W>(self, other: Po<V, W>) -> Po<T, W> {
        Po {
            x: self.x,
            y: other.y,
        }
    }
}

// 泛型代码的性能
// 使用泛型代码的性能不会比使用具体类型的性能差
// 单态化：编译器在编译时会将泛型代码转换为针对具体类型的代码
