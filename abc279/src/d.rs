use num::ToPrimitive;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn main() {
    input! {
        a: isize,
        b: isize
    }

    let bf = b.to_f64().unwrap();
    let af = a.to_f64().unwrap();

    println!("{}", local_minimum(0, 10isize.pow(14), |x| {
        let xf = x.to_f64().unwrap();
        bf*xf + af/(xf+1.0).sqrt()
    }));
}

fn local_minimum<F>(lower: isize, upper: isize, f: F) -> f64 where
F: Fn(isize) -> f64 {
    let mut left = lower;
    let mut right = upper;

    while (left+2) < right {
        let mid_l = left + (right-left) / 3;
        let mid_r = right - (right-left) / 3;

        if f(mid_l) < f(mid_r) {
            right = mid_r;
        } else {
            left = mid_l;
        }
    }

    return f(left).min(f(left+1)).min(f(right));
}
