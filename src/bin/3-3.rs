fn main() {
    // 复合类型
    // Tuple
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式
    // 元组长度固定：一旦声明，其长度不会增长或缩短
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构元组
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    // 通过点号加索引来访问元组中的元素
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // Array
    // 数组是一个将多个相同类型的值组合进一个复合类型的主要方式
    // 数组中的每个元素的类型必须相同
    // 数组的长度在编译时就必须已知
    // 数组是分配在栈上的
    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];
    // 数组的类型是 [T; N]，其中 T 是数组中元素的类型，N 是数组的长度
    // 数组的用处
    // 1. 如果你想要在堆上而不是栈上分配数据，可以使用向量（vector）
    // 2. 如果你想要在任何给定时间只拥有一个值，可以使用字符串（string）
    // 3. 如果你想要拥有多个值，但是这些值的类型相同，可以使用数组
    // 4. 如果你想要拥有多个值，而且这些值的类型不同，可以使用元组
    // 访问数组元素
    let first = a[0];
    // let index = 10;
    // let x = a[index];
    let index = [10, 11, 12];
    let x = a[index[0]];
    // 运行时访问数组元素越界会导致程序崩溃
    println!("{}, {}", first, x);
}
