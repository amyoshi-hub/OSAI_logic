use std::f32::consts::PI;

fn hello1(x: i32) -> i32 {
    println!("Hello 1! x = {}", x);
    10 * x
}

fn hello2(x: i32) -> i32 {
    println!("Hello 2! x = {}", x);
    10 * x
}

fn hello3(x: i32) -> i32 {
    println!("Hello 3! x = {}", x);
    10 * x
}

fn max_zero(x: i32) -> i32 {
    // if文を使わない max(0, x)
    (x + x.abs()) / 2
}

fn simd_like_dispatch(weights: [i32; 3]) {
    let weights = [
        max_zero(weights[0]),
        max_zero(weights[1]),
        max_zero(weights[2]),
    ];

    let _ = hello1(weights[0]);
    let _ = hello2(weights[1]);
    let _ = hello3(weights[2]);
}

fn main() {
    let mut weights = [0; 3];

    for i in 0..100 {
        let x = i as f32 * 0.1;
        let value = ((x.sin() + 1.0) / 2.0) * 3.0;
        let idx = (value as usize).min(2); // if文使わず saturating_sub でも可
        weights[idx] += 1;
    }

    println!("weights: {:?}", weights);
    simd_like_dispatch(weights);
}

