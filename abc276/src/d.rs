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
        mut arr: [usize; n],
    }

    let mut facts2 = vec![0usize; n];
    let mut facts3 = vec![0usize; n];

    for i in 0..arr.len() {
        while arr[i]%2 == 0 {
            facts2[i] += 1;
            arr[i] /= 2;
        }
        while arr[i]%3 == 0 {
            facts3[i] += 1;
            arr[i] /= 3;
        }
    }

    for i in 0..arr.len()-1 {
        if (arr[i] != arr[i+1]) {
            println!("{}", -1);
            return;
        }
    }

    let minfact2 = facts2.iter().min().unwrap();
    let minfact3 = facts3.iter().min().unwrap();



    let mut result = 0usize;

    result += facts2.iter().map(|&c| c - minfact2).sum::<usize>();
    result += facts3.iter().map(|&c| c - minfact3).sum::<usize>();

    println!("{}", result);
}
