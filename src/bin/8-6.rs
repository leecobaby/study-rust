// 更新 HashMap<K, V> 的值
// HashMap 大小可变，可以随时增加或减少键值对
// HashMap 中的值可以被覆盖
// 更新 HashMap<K, V> 情况
// 1. 如果键值对不存在，插入新的键值对
// 2. 如果键值对存在，更新键值对的值
// 2.1 替换旧值
// 2.2 保留旧值，不更新
// 2.3 合并旧值和新值

// Entry API
// Entry API 提供了一种更加灵活的方式来处理 HashMap 中的键值对
// Entry API 有三种情况：
// 1. Entry::Vacant
// 2. Entry::Occupied
// 3. Entry::Vacant 和 Entry::Occupied
// 返回：
// 如果Key存在，返回到对应的Value的可变引用
// 如果Key不存在，返回到一个新插入的Value的可变引用
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 1. 如果键值对不存在，插入新的键值对
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);
    // 2. 如果键值对存在，更新键值对的值
    // 保留旧值，不更新
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    // 更新旧值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

// Hash 函数
// 默认情况下，HashMap 使用加密功能强大的Hash函数，可以抵抗拒绝服务（DoS）工具
// 不是可用的最快的 Hash 算法
// 但具有更好额安全性
// 可以指定不同的 hasher 来切换到另一个函数
