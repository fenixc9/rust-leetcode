#![feature(portable_simd)]

use std::simd::{i32x4, SimdFloat};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
const COUNT: usize = 10000000;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn criterion_benchmark1(c: &mut Criterion) {
    let numbers: Vec<f64> = (1..COUNT).map(|x| x as _).collect();
    c.bench_function("simd sum", |b| b.iter(|| sum_a(&numbers)));
}
fn criterion_benchmark2(c: &mut Criterion) {
    let numbers: Vec<f64> = (1..COUNT).map(|x| x as _).collect();
    c.bench_function("normal sum", |b| b.iter(|| &basic_simd_sum));
}

#[test]
fn f1() {

    let a = i32x4::from_array([1, 1, 3, 3]);
    let b = i32x4::from_array([2, 2, 0, 0]);

// ge: 大于等于计算，生成bool元素类型的向量
    let m = a.ge(&b);

    dbg!(m);
}


criterion_group!(benches1, criterion_benchmark,criterion_benchmark1,criterion_benchmark2);
// criterion_group!(benches2, criterion_benchmark1);
// criterion_group!(benches3, criterion_benchmark2);
criterion_main!(benches1);

fn sum_a(x: &[f64]) -> f64 {
    return x.iter().copied().sum();
}

fn basic_simd_sum(x: &[f64]) -> f64 {
    use std::ops::Add;
    use std::simd::f64x4;
    let (prefix, middle, suffix) = x.as_simd();

    let sums = f64x4::from_array([
        prefix.iter().copied().sum(),
        0.0,
        0.0,
        suffix.iter().copied().sum(),
    ]);
    let sums = middle.iter().copied().fold(sums, f64x4::add);
    sums.reduce_sum()
}
