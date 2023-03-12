// Trait
// 告诉编译器某种类型具有哪些并且可以与其他类型共享的功能
// 通过 trait 定义共享的行为
// 通过 trait bound 指定泛型可以是任何拥有特定行为的类型

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
}