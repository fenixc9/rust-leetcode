// extern crate test;

use std::simd::{f32x4, i32x4, SimdFloat, SimdInt, StdFloat};
use std::thread::sleep;
use std::time::Duration;

const COUNT: usize = 10000000;

#[allow(dead_code)]
#[test]
// #[bench]
fn f1() {
    let numbers: Vec<f64> = (1..COUNT).map(|x| x as _).collect();


    let sum = basic_simd_sum(&numbers[..]);
    sleep(Duration::from_millis(100));


    let r = sum_a(&numbers);
}

#[test]
fn f2() {
    let numbers: Vec<f64> = (1..COUNT).map(|x| x as _).collect();
}

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


#[test]
fn f3() {
    let a = &(0..16).map(|e|e as f32).collect::<Vec<f32>>();
    let b = &(0..16).map(|e|e as f32).collect::<Vec<f32>>();
    // let map = a.array_chunks::<4>()
    //     .map(|&a| i32x4::from_array(a))
    //     .zip(b.array_chunks::<4>().map(|&b| i32x4::from_array(b)))
    //     .map(|(a, b)| (a * b).reduce_sum())
    //     .sum::<i32>();
    // dbg!(map);
    //

    // let i = a.array_chunks::<4>()
    //     .map(|&a| i32x4::from_array(a))
    //     .zip(b.array_chunks::<4>().map(|&b| i32x4::from_array(b)))
    //     .fold(i32x4::splat(0), |acc, zipped| acc + zipped.0 * zipped.1)
    //     .reduce_sum();

    // let simd = f32x4::splat(0.0);

    let mut res = f32x4::splat(0.0);
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .for_each(|(a, b)| {
            res = a.mul_add(b, res);
        });


    dbg!(res.reduce_sum());
}

#[test]
fn t1() {
    let a = &(0..12).map(|e|e as f32).collect::<Vec<f32>>();
    let b = &(0..13).map(|e|e as f32).collect::<Vec<f32>>();
    assert_eq!(&dot_prod_simd_4(&a, &a),&dot_prod_simd_4(&b, &b));
}

pub fn dot_prod_simd_2(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    // TODO handle remainder when a.len() % 4 != 0
    let mut res = f32x4::splat(0.0);
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .for_each(|(a, b)| {
            res = a.mul_add(b, res);
        });
    res.reduce_sum()
}

pub fn dot_prod_simd_5(a: &[f32], b: &[f32]) -> f32 {
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .fold(f32x4::splat(0.), |acc, (a, b)| a.mul_add(b, acc))
        .reduce_sum()
}

pub fn dot_prod_simd_4(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = a
        .array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .map(|(a, b)| a * b)
        .fold(f32x4::splat(0.0), std::ops::Add::add)
        .reduce_sum();
    let remain = a.len() - (a.len() % 4);
    sum += a[remain..]
        .iter()
        .zip(&b[remain..])
        .map(|(a, b)| a * b)
        .sum::<f32>();
    sum
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(4, add_two(2));
//     }
//
//     #[bench]
//     fn bench_add_two(b: &mut Bencher) {
//         b.iter(|| add_two(2));
//     }
// }