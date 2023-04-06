use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};

fn bsearch<F>(l: usize, sat: F) -> Option<usize> 
where F: Fn(usize) -> bool {
    let mut ok: isize = l as isize;
    let mut ng: isize = -1;

    while (ok-ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if sat(mid as usize) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    return if ok == l as isize { None } else { Some(ok as usize) };
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ps: [Usize1; n],
    }

    let mut eaten_at = vec![-1 as isize; n];
    let mut under_card: Vec<isize> = vec![-1; n];
    let mut stack_num: Vec<usize> = vec![0; n];
    let mut field: BTreeSet<usize> = BTreeSet::new();

    for i in 0..ps.len() {
        if let Some(&c) = field.range(ps[i]+1..).next() {
            field.remove(&c);
            under_card[ps[i]] = c as isize;
            stack_num[ps[i]] = stack_num[c]+1;

            if stack_num[ps[i]] < k {
                field.insert(ps[i]);
            } else if stack_num[ps[i]] >= k {
                let mut j = ps[i] as isize;
                while j != -1 {
                    eaten_at[j as usize] = i as isize + 1;
                    j = under_card[j as usize];
                }
                field.remove(&ps[i]);
            }
        } else {
            if k == 1 {
                eaten_at[ps[i]] = i as isize + 1;
            } else {
                field.insert(ps[i]);
                stack_num[ps[i]] = 1;
            }
        }
    }
    eaten_at.iter().for_each(|&v| {
        println!("{}", v);
    });
}
