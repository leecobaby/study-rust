// 路径 Path
// 为了在 Rust 的模块系统中引用一个项，需要指定它的路径

// 绝对路径
// 从 crate root 开始，使用 crate 名称或字面值 crate 来指定
// 例如：
// crate::front_of_house::hosting::add_to_waitlist();

// 相对路径
// 从当前模块开始，使用 self、super 或当前模块的标识符来指定
// 例如：
// front_of_house::hosting::add_to_waitlist();

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 私有边界
// 模块不仅可以组织代码，还可以定义私有边界
// 如果想把函数或 struct 变成私有的，需要将其放在模块中
// rust 中所有条目默认都是私有的
// 父级模块无法访问子模块中的私有条目

// 为了在父级模块中使用子模块中的公有条目，需要在父级模块中使用 pub 关键字将子模块声明为公有的
