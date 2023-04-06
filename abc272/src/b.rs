use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, HashMap};


fn main() {
    input! {
        n: usize,
        m: usize,
        r: [[Usize1]; m]
    }

    let mut paired: HashMap<(usize, usize), bool> = HashMap::new();
    for i in 0..n {
        for j in i+1..n {
            paired.insert((i, j), false);
        }
    }

    for ps in r {
        for i in 0..ps.len() {
            for j in i+1..ps.len() {
                paired.insert((ps[i].min(ps[j]), ps[i].max(ps[j])), true);
            }
        }
    }

    let result = paired.values().all(|&x| x);
    println!("{}", if result { "Yes" } else { "No" });
}
