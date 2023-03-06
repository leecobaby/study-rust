fn main() {
    let v = Some(0u8);

    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let 语法糖
    if let Some(3) = v {
        println!("three");
    } else {
        println!("not three {}", v.unwrap());
    }
}
