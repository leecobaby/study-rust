// 迭代器
// 对一系列项执行操作
// rust 迭代器是惰性的，只有调用时才会执行

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// 所有迭代器都实现了 Iterator trait
// Iterator trait 只有一个方法：next
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
// type item 和 self::item 定义了与该 trait 关联的类型
// next 方法定义了迭代器的行为
// next 方法返回一个 Option<Self::Item>，其中 Self::Item 是迭代器的元素类型
// next 方法的签名表明了迭代器是可变的（&mut self），这是因为迭代器的状态可能会随着迭代而改变

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}

// 几个迭代方法
// iter - 不可变引用
// into_iter - 获取所有权
// iter_mut - 可变引用
