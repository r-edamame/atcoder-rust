use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().sum::<usize>());
}
