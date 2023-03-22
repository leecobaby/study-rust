// 性能比较：循环 vs 迭代器
// 迭代器的性能更好

// 零开销抽象 (zero-cost abstraction)

// 音频解码器例子
fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
