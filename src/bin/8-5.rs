// HashMap<K, V>
// 键值对的形式存储数据，键值对的类型为 K 和 V
// 适用场景：当需要根据键快速查找值时，HashMap 是一个很好的选择
// 同构：所有的键和值的类型相同
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap 所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    map.insert(&field_name, &field_value);
    println!("{}", field_name);

    // 访问 HashMap 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(v) => println!("{}", v),
        None => println!("None"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Green")).or_insert(50);
    // println!("{:?}", scores);

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);
}
