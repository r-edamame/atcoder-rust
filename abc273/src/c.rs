use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};


fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>
where F: Fn(isize) -> bool
{
    let mut ng = lower-1;
    let mut ok = upper+1;
    while (ng-ok).abs() > 1 {
        let mid = (ng+ok) / 2;
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
        alist: [usize; n],
    }

    let mut bs = alist.clone();
    bs.sort();
    bs.dedup();
    
    let mut r = vec![0; n];
    
    for a in alist {
        let ix = bsearch(0, bs.len() as isize, |i| a <= bs[i as usize]).unwrap();
        r[bs.len() - 1 - ix as usize] += 1;
    }

    r.iter().for_each(|k| {
        println!("{}", k);
    });
}
