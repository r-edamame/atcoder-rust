use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet, BTreeMap};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn main() {
    input! {
        n: usize,
        ls: [(usize, usize); n],
    }

    let mut g: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for (f, t) in ls {
        g.entry(f).and_modify(|to| to.push(t)).or_insert(vec![t]);
        g.entry(t).and_modify(|to| to.push(f)).or_insert(vec![f]);
    }

    let mut passed: BTreeSet<usize> = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut max = 1usize;
    queue.push_back(1usize);
    while let Some(p) = queue.pop_front() {
        if passed.contains(&p) {
            continue;
        }

        if p > max {
            max = p;
        }

        for q in g.get(&p).unwrap_or(&vec![]) {
            queue.push_back(*q);
        }

        passed.insert(p);
    }

    println!("{}", max);
}
