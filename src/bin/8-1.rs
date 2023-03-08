// Vector
// Vec<T>
// 由标准库提供的一个泛型类型，可以存储任意类型的值
// 通过 Vec::new 创建一个空的 Vec<T>
// 只能存储相同类型的值
// 值在内存中连续存放的

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);

    // 读取 越界 会 panic
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 这种方式会更安全 越界会返回 None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 可变借用和不可变借用不允许同时存在
    let first = &v[0];
    // v.push(6);
    println!("The first element is: {}", first);

    // 遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // 解引用，修改值
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}
