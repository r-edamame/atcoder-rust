use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet, BTreeMap};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn lower_bound<F>(lower: isize, upper: isize, sat: F) -> Option<isize> where F: Fn(isize) -> bool {
    let mut ok = lower-1;
    let mut ng = upper+1;

    while (ok-ng).abs() > 1 {
        let mid = (ok+ng) / 2;
        if sat(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return if ok == lower-1 { None } else { Some(ok) };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        qs: [(Usize1, Usize1, usize); q],
    }

    let mut appear: Vec<Vec<usize>> = vec![vec![]; 2 * 100000 + 1];
    for (i, &v) in a.iter().enumerate() {
        appear[v].push(i);
    }

    for (l, r, x) in qs {
        if appear[x].len() == 0 {
            println!("0");
            continue;
        }
        let rc = lower_bound(0, (appear[x].len()-1) as isize, |i| appear[x][i as usize] <= r).unwrap_or(-1);
        let lc = lower_bound(0, (appear[x].len()-1) as isize, |i| appear[x][i as usize] < l).unwrap_or(-1);

        println!("{}", rc-lc);
    }
}
