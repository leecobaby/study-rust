// Rust 错误处理

// 不可恢复的错误与 panic!
// 当 panic! 宏执行：
// - 你的程序会打印一个错误信息
// - 展开（unwind）、清理调用栈（stack）
// - 退出程序

// 展开调用栈 工作量大，会清理内存
// 立即终止调用栈，不进行清理，立即停止
// 想让二进制文件更小，把设置的从“展开”改成“终止”
// - 在 Cargo.toml中适当的 profile 部分设置
// panic = 'abort'
fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}

// 查询 panic! 产生的回溯信息
// - RUST_BACKTRACE=1 cargo run
