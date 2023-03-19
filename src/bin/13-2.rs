fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    // error: 闭包的类型在第一次推断后就不会再改变
    let n = example_closure(5);
}
