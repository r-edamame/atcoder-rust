use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};

const DISTANCE_MAX: usize = 4000000000;

fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>
where F: Fn(isize) -> bool {
    let mut ok: isize = upper+1;
    let mut ng: isize = lower-1;
    while (ok-ng).abs() > 1 {
        let mid = (ok+ng)/2;
        if sat(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == upper+1 { None } else { Some(ok) }
}

type JumpStand = ((isize, isize), isize);

fn main() {
    input! {
        n: usize,
        js: [((isize, isize), isize); n],
    }

    let result = bsearch(1, DISTANCE_MAX as isize, |i| {
        let mut to = vec![vec![]; n];
        // 連結グラフの作成
        for j in 0..n {
            for k in 0..n {
                if j == k {
                    continue;
                }
                if reach(i as usize, js[j], js[k].0) {
                    to[j].push(k);
                }
            }
        }
        println!("s: {}", i);
        print_to(&to);
        (0..n).any(|l| connected(&to, l, n))
    }).unwrap();

    println!("{}", result);
}

fn print_to(to: &Vec<Vec<usize>>) {
    for i in 0..to.len() {
        let ps = &to[i];
        print!("    from {}: ", i);
        println!("{}", ps.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn connected(to: &Vec<Vec<usize>>, from: usize, len: usize) -> bool {
    let mut passed = vec![false; len];
    let mut q = VecDeque::new();
    let mut count = 0;
    q.push_back(from);
    while let Some(i) = q.pop_front() {
        if passed[i] {
            continue;
        }

        for &t in &to[i] {
            q.push_back(t);
        }
        passed[i] = true;
        count += 1;
    }
    count == len
}

fn reach(s: usize, ((x1, y1), p1): JumpStand, (x2, y2): (isize, isize)) -> bool {
    p1*(s as isize) >= (x1-x2).abs() + (y1-y2).abs()
}
