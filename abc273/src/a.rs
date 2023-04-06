use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize
    }
    println!("{}", fact(n));
}

fn fact(n: usize) -> usize {
    if n == 0 { 1 } else { n * fact(n-1) }
}
