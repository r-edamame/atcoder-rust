use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    let r = (x*n).min((n/3*y) + (n%3*x));
    println!("{}", r);
}
