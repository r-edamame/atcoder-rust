use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};
use std::ops::Add;

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}


fn transpose((h, m): (usize, usize)) -> (usize, usize) {
    let ht = h / 10;
    let hb = h % 10;
    let mt = m / 10;
    let mb = m % 10;

    return (ht*10 + mt, hb*10 + mb);
}
fn valid_time((h, m): (usize, usize)) -> bool {
    0 <= h && h < 24 && 0 <= m && m < 60
}
fn increment((h, m): (usize, usize)) -> (usize, usize) {
    let carry = (m+1) / 60;
    ((h+carry)%24, (m+1)%60)
}

fn main() {
    input! {
        h: usize,
        m: usize,
    }

    let mut hm = (h, m);
    while !valid_time(transpose(hm)) {
        hm = increment(hm);
    }

    println!("{} {}", hm.0, hm.1);
}
