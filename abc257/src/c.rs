use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        s: Chars,
        ws: [usize; n],
    }

    let otona_num = s.iter().filter(|&&c| c == '1').count();

    let mut ps: Vec<(bool, usize)> = s.iter().map(|&s| s == '1').zip(ws).collect();
    ps.sort_by_key(|p| p.1);

    let mut correct = otona_num;
    let mut max_correct = otona_num;
    for i in 0..n {
        if ps[i].0 {
            correct -= 1;
        } else {
            correct += 1;
        }

        // 同じ体重が続いている間は最大値の判定をしない
        if i == n-1 || ps[i].1 != ps[i+1].1 {
            max_correct = max_correct.max(correct);
        }
    }

    println!("{}", max_correct);
}
