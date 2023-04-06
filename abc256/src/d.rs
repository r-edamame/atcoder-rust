use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
    }

    let mut imos: Vec<isize> = vec![0; 200001];
    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        imos[l] += 1;
        imos[r] -= 1;
    }

    let mut current = 0;
    for i in 1..=200000 {
        if imos[i] == 0 {
            continue;
        }

        let next = current + imos[i];

        if current == 0 && next > 0 {
            print!("{} ", i);
        } else if current > 0 && next == 0 {
            println!("{}", i);
        }

        current = next;
    }
}
