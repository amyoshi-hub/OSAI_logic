#![feature(portable_simd)]

use std::simd::{Simd};
use std::simd::num::SimdFloat;  // SimdFloatトレイトをインポート

use std::time::Instant;
use std::simd::StdFloat;

fn hello1(x: usize) { println!("hello1: {}", x); }
fn hello2(x: usize) { println!("hello2: {}", x); }
fn hello3(x: usize) { println!("hello3: {}", x); }

// マクロでif-elseを生成
macro_rules! gen_dispatch {
    // ベースケース：0個
    () => {
        compile_error!("関数が1つ以上必要です");
    };

    // 1個の関数
    ($f1:ident) => {
        fn dispatch(i: usize) {
            $f1(i);
        }
    };

    // 2個以上は再帰的に展開
    ($f1:ident, $($rest:ident),+) => {
        fn dispatch(i: usize) {
            if i % count_idents!($f1, $($rest),+) == 0 {
                $f1(i);
            } else {
                gen_dispatch!(@else i, 1, [$($rest),+]);
            }
        }
    };

    // else節の展開補助マクロ
    (@else $i:ident, $idx:expr, [$f:ident $(, $rest:ident)*]) => {
        if $i % count_idents!($f $(, $rest)*) == $idx {
            $f($i);
        } else {
            gen_dispatch!(@else $i, $idx + 1, [$($rest),*]);
        }
    };

    (@else $i:ident, $idx:expr, []) => {
        // 何もしない（すべてカバーされている）
    };
}

// 識別子の数を数えるマクロ
macro_rules! count_idents {
    ($($idents:ident),*) => {
        <[()]>::len(&[$(count_idents!(@sub $idents)),*])
    };
    (@sub $idents:ident) => { () };
}

// マクロ展開例
gen_dispatch!(hello1, hello2, hello3);

fn main() {
    for i in 0..10 {
        dispatch(i);
    }
}

fn simd_version_dispatch(weights: &mut [i32; 3], i: usize) {
    let step = 0.1;
    let base = [
        (i + 0) as f32 * step,
        (i + 1) as f32 * step,
        (i + 2) as f32 * step,
        (i + 3) as f32 * step,
    ];
    let angles = Simd::<f32, 4>::from_array(base);

    let sines = angles.sin();
    let scaled = ((sines + Simd::splat(1.0)) / Simd::splat(2.0)) * Simd::splat(3.0);

    // castメソッドはSimdFloatトレイトの一部なので、import必須
    let indices: [usize; 4] = scaled.cast::<usize>().to_array();

    for &i in &indices {
        let idx = i.min(2);
        weights[idx] += 1;
    }

    let _ = hello1(weights[0]);
    let _ = hello2(weights[1]);
    let _ = hello3(weights[2]);
}

fn max_zero(x: i32) -> i32 {
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

    let start = Instant::now();
    for i in (0..100).step_by(4) {
        simd_version_dispatch(&mut weights, i);
    }
    let duration = start.elapsed();

    let start2 = Instant::now();
    for i in 0..100 {
        if i % 3 == 0 {
            hello1(i);
        } else if i % 3 == 1 {
            hello2(i);
        } else {
            hello3(i);
        }
    }
    let duration2 = start2.elapsed();
    for i in 0..10 {
        dispatch(i);
    }
    println!("SIMD version execution time: {:?}", duration);
    println!("If-branch version execution time: {:?}", duration2);
}

