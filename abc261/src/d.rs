use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n],
        cys: [(usize, usize); m],
    }

    let mut bonus = vec![0; n+1];
    for (c, y) in cys {
        bonus[c] = y;
    }

    // dp[i][j] = i回目のコイントスでカウンタがjになるときにもらえるお金の最大値
    let mut dp: Vec<Vec<usize>> = vec![];
    for i in 1..=n+1 {
        dp.push(vec![0; i]);
    }

    for i in 1..=n {
        dp[i][0] = (0..i).map(|l| dp[i-1][l]).max().unwrap();
        for j in 1..=i {
            dp[i][j] = dp[i-1][j-1] + xs[i-1] + bonus[j];
        }
    }

    let result = (0..=n).map(|i| dp[n][i]).max().unwrap();

    println!("{}", result);
}
