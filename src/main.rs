#![feature(let_chains)]

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
