use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    // i番目までの間でxを選んだ際の最小値
    let mut pl = [vec![0], a.clone()].concat();
    for i in 0..n {
        pl[i+1] = (l*(i+1) as i64).min(pl[i]+pl[i+1]);
    }

    // yを選んだ時の変分
    let mut pr = [a.clone(), vec![0]].concat();
    pr.reverse();
    for i in 0..n {
        pr[i+1] = (r*(i+1) as i64).min(pr[i]+pr[i+1]);
    }

    let mut min = std::i64::MAX;
    for k in 0..=n {
        min = min.min(pl[k]+pr[n-k]);
    }

    println!("{}", min);
}
