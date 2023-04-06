use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn dfs(to: &Vec<Vec<usize>>, from: usize, target: usize) -> bool {
    let mut q = VecDeque::new();
    let mut passed = vec![false; to.len()];

    q.push_back(from);
    while let Some(p) = q.pop_front() {
        if p == target {
            return true;
        }
        if passed[p] {
            continue;
        }
        passed[p] = true;

        for &t in &to[p] {
            q.push_back(t);
        }
    }
    return false;
}

fn main() {
    input! {
        n: usize,
        s: (isize, isize),
        t: (isize, isize),
        circles: [(isize, isize, usize); n]
    }

    let s_in = circles.iter().position(|&(x, y, r)| (s.0-x).pow(2) + (s.1-y).pow(2) == (r as isize).pow(2)).unwrap();
    let t_in = circles.iter().position(|&(x, y, r)| (t.0-x).pow(2) + (t.1-y).pow(2) == (r as isize).pow(2)).unwrap();

    let mut to: Vec<Vec<usize>> = vec![vec![]; n];

    // 円同士のつながりのグラフを作成する
    for i in 0..circles.len() {
        for j in i+1..circles.len() {
            if crossed(&circles[i], &circles[j]) {
                to[i].push(j);
                to[j].push(i);
            }
        }
    }

    let result = dfs(&to, s_in, t_in);
    println!("{}", if result { "Yes" } else { "No" });
}

fn crossed(&(x1, y1, r1): &(isize, isize, usize), &(x2, y2, r2): &(isize, isize, usize)) -> bool {
    let r1i = r1 as isize;
    let r2i = r2 as isize;
    (x1-x2).pow(2) + (y1-y2).pow(2) <= (r1i+r2i).pow(2) &&
    (x1-x2).pow(2) + (y1-y2).pow(2) >= (r1i-r2i).pow(2)
}
