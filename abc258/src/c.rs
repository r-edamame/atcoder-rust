use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: isize,
        q: isize,
        s: Chars,
        queries: [(usize, isize); q]
    }

    let mut current = 0;
    for (i, num) in queries {
        match i {
            1 => {
                current = (current-num).rem_euclid(n);
            }
            2 => {
                println!("{}", s[((current + num-1) % n) as usize])
            }
            _ => panic!()
        }
    }
}
