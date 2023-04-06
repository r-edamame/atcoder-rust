use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![]; 10]; 10];

    for i in 0..10 {
        for j in 1..=i+1 {
            dp[0][i].push(vec![j]);
        }
    }

    for i in 1..n {
        for k in 1..m {
            if i <= k {
                for mut seq in dp[i-1][k-1].clone() {
                    seq.push(k+1);
                    dp[i][k].push(seq);
                }
                for seq in dp[i][k-1].clone() {
                    dp[i][k].push(seq);
                }
            }
        }
    }

    let results = &mut dp[n-1][m-1];
    results.sort();
    results.iter().for_each(|seq| { print_seq(&seq) });

}

fn print_seq(v: &Vec<usize>) {
    println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}