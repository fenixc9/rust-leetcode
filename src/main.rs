#![feature(let_chains)]
#![feature(portable_simd)]
#![feature(array_chunks)]
// #![feature(portable_simd)]

use std::collections::VecDeque;

mod leetcode;
mod common;

fn main() {
    let mut q = VecDeque::new();
    q.push_front(1);
    q.push_front(2);
    q.push_front(3);

    while let Some(n) = q.front() && n > &1 {
        println!("{}",n);
        q.pop_front();
    }
}
