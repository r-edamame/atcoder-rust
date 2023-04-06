use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};

fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>
where F: Fn(isize) -> bool
{
    let mut ng = lower-1;
    let mut ok = upper+1;
    while (ng-ok).abs() > 1 {
        let mid = (ng+ok)/2;
        if sat(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return if ok == upper+1 { None } else { Some(ok) };
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut as_: [isize; n],
        xs: [isize; q]
    }

    as_.sort();
    let mut assocs = vec![0; n];
    assocs[0] = as_[0];
    for i in 0..n-1 {
        assocs[i+1] = assocs[i] + as_[i+1];
    }

    for x in xs {
        let r = if x <= as_[0] {
            assocs[n-1] - x * (n as isize)
        } else if x >= as_[n-1] {
            x*(n as isize) - assocs[n-1]
        } else {
            let i = bsearch(0, (n as isize)-1, |i| x <= as_[i as usize]).unwrap();
            (x * i - assocs[(i as usize)-1]) + (assocs[n-1]-assocs[(i as usize)-1] - x*((n as isize) - i))
        };

        println!("{}", r);
    }
}
