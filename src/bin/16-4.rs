// send 、sync trait
// 在rust语言中有两个并发概念：
// std::marker::Send 和 std::marker::Sync
// Rust 中几乎所有的类型都实现了 send
// 但 Rc<T> 和 RefCell<T> 这两个类型没有实现 send，它只用于单线程场景
// Sync ：多个线程可以同时访问
// 也就是说，如果一个类型 T 实现了 Sync，那么 &T 就是 Send
