use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        mut x: usize,
        k: usize
    }

    let mut d = 1;
    for i in 0..k {
        let dn = (x / d) % 10;
        if dn < 5 {
            x -= d * dn;
        } else {
            x += d * (10 - dn);
        }
        d *= 10;
    }

    println!("{}", x);
}
