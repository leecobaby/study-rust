// Trait
// 告诉编译器某种类型具有哪些并且可以与其他类型共享的功能
// 通过 trait 定义共享的行为
// 通过 trait bound 指定泛型可以是任何拥有特定行为的类型

use std::fmt::Display;

// 定义一个包含行为的 trait
pub trait Summary {
    // fn summarize(&self) -> String;
    // 它可以是抽象的签名也可以是默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)");
        format!("(Read more from {}...)", self.summarize_author())
    }

    // 默认实现可以调用其他方法
    fn summarize_author(&self) -> String;
}

// 在类型上实现 trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.content)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let str_list = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ];
    let result = largest(&str_list);
    println!("The largest str is {}", result);
}

// Trait 作为参数
pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound
pub fn notify_pro<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with where
pub fn notify_where<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summarize());
}

// Trait 作为返回类型
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        // 要比较大小，需要提供比较功能的 trait
        // std::cmp::PartialOrd
        if item > &largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 条件 trait bound
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
