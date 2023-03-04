// struct
// 结构体
// 自定义的数据类型

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct
// 元组结构体
// 元组结构体是没有命名字段的元组
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
// 单元结构体
// 没有任何字段的结构体
struct UnitLikeStruct;

// Struct 所有权
// 结构体的实例是它们的所有者
// 结构体数据的每个字段都是它们的所有者
// 当结构体的实例离开作用域时，其所有字段也将被丢弃
// 但是，如果结构体包含引用，则数据将不会被丢弃
// 因为引用的数据不是结构体的所有者
// 但是，结构体实例本身仍将被丢弃
// 因为结构体实例是结构体的所有者
// 如果结构体包含引用，则需要使用生命周期注解
// struct User1 {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }
fn main() {
    let mut user1 = User {
        email: String::from("abc@example.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("xyz@example.com");
}
