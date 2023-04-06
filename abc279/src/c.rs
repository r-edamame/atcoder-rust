use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet, HashMap, BTreeMap};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn main() {
    input! {
        h: usize,
        w: usize,
        _ss: [Chars; h],
        _ts: [Chars; h],
    }

    let mut ss: Vec<Vec<char>> = Vec::with_capacity(w);
    let mut ts: Vec<Vec<char>> = Vec::with_capacity(w);

    for i in 0..w {
        let mut s = Vec::with_capacity(h);
        let mut t = Vec::with_capacity(h);

        for j in 0..h {
            s.push(_ss[j][i]);
            t.push(_ts[j][i]);
        }

        ss.push(s);
        ts.push(t);
    }

    let mut count_s: BTreeMap<String, usize> = BTreeMap::new();
    let mut count_t: BTreeMap<String, usize> = BTreeMap::new();

    for i in 0..w {
        count_s.entry(ss[i].iter().collect())
            .and_modify(|c| { *c += 1 })
            .or_insert(1);
        count_t.entry(ts[i].iter().collect())
            .and_modify(|c| { *c += 1 })
            .or_insert(1);
    }

    if ss.len() != ts.len() {
        println!("No");
        return;
    }

    for (sc, tc) in count_s.iter().zip(count_t.iter()) {
        if *sc.0 != *tc.0 || sc.1 != tc.1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
