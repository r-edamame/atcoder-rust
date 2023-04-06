use num::Integer;
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
        n: usize,
        a: usize,
        b: usize,
    }

    let lcm = a.lcm(&b);
    let na = n/a;
    let nb = n/b;
    let nab = n/lcm;

    let r = if a == 1 || b == 1 {
        0
    } else if a == b {
        ((n+1)*n/2) - ((na+1)*a*na/2)
    } else {
        ((n+1)*n/2) - ((na+1)*a*na/2) - ((nb+1)*b*nb/2) + ((nab+1)*lcm*nab/2)
    };
    println!("{}", r);
}
