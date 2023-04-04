// 模式匹配两种形式: 可辩驳的和不可辩驳的
fn main() {
    let a = Some(5);
    let Some(x) = a;
    if let Some(x) = a {
        println!("{}", x);
    }
}
