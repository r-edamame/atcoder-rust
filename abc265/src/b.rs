use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        n: i64,
        m: i64,
        mut t: i64,
        a: [i64; n-1],
        b: [(Usize1,i64); m],
    }

    let mut bonus: Vec<i64> = vec![0; n as usize];
    for (x, p) in b {
        bonus[x] = p;
    }

    let mut i = 0;
    while t > 0 && i < n-1 {
        if a[i as usize] >= t {
            break;
        }
        t -= a[i as usize];
        i += 1;
        t += bonus[i as usize];
        println!("{}", t);
    }

    println!("{}", if i == n-1 {"Yes"} else {"No"});
}
