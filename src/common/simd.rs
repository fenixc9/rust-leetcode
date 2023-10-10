use std::simd::Simd;

#[test]
fn f1() {
    let s4 = Simd::from([1, 2, 3, 4]);
    let s2 = Simd::from([0, 1, 1, 1]);
    // let simd1 = std::simd::Simd([3, 2, 3, 4]);
    // let simd2 = std::simd::Simd([1, 2, 5, 10]);

    let simd3: Simd<i32, 4> = s4 + s2;
    let x: &[i32; 4] = simd3.as_array();
    println!("{:?}", x);
}