// 产生其他迭代器的方法
// 比如 map 会产生一个新的迭代器，它会调用一个闭包来处理每个元素


#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        // _ 表示让编译器推断出来里面的元素类型
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}