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
        h: usize,
        w: usize,
        mat: [[usize; w]; h],
    }

    let mut passed = BTreeSet::new();

    println!("{}", dfs(w, h, &mat, 0, 0, &mut passed));
}

fn dfs(w: usize, h: usize, mat: &Vec<Vec<usize>>, x: usize, y: usize, passed: &mut BTreeSet<usize>) -> usize {
    let v = mat[y][x];
    if !passed.insert(v) {
        return 0;
    }

    if x == w-1 && y == h-1 {
        passed.remove(&v);
        return 1;
    }

    let r = if x == w-1 { 0 } else { dfs(w, h, mat, x+1, y, passed) };
    let d = if y == h-1 { 0 } else { dfs(w, h, mat, x, y+1, passed)};

    passed.remove(&v);
    return r + d;
}
