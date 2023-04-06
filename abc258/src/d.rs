use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut min_to_first_clear = vec![0; n+1];
    for i in 1..=n {
        let (a, b) = ab[i-1];
        min_to_first_clear[i] = min_to_first_clear[i-1] + a + b;
    }

    let mut min_to_clear_x = vec![0; n];
    for i in 0..n {
        let (_a, b) = ab[i];
        if (i+1) > x {
            min_to_clear_x[i] = std::usize::MAX;
            continue;
        }
        min_to_clear_x[i] = min_to_first_clear[i+1] + (x-(i+1)) * b;
    }

    println!("{}", min_to_clear_x.iter().min().unwrap());
}
