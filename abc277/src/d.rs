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
        m: usize,
        vs: [usize; n],
    }

    let mut total: usize = 0;
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    for v in vs {
        total += v;
        map.entry(v).and_modify(|c| *c += 1).or_insert(1);
    }

    let keys: Vec<&usize> = map.keys().collect::<Vec<&usize>>();
    let kinds = keys.len();

    if kinds == m {
        println!("{}", 0);
        return;
    }

    let mut p = 0;
    for i in 0..kinds {
        if (keys[i]+1%m) != *keys[(i+1)%kinds] {
            p = i;
            break;
        }
    }

    let mut s = vec![total; kinds];
    for i in 0..kinds {
        let j = (p+kinds-i)%kinds;
        let t = keys[j] * map.get(keys[j]).unwrap();
        if (keys[j]+1)%m == *keys[(j+1)%kinds] {
            s[j] = s[(j+1)%kinds] - t;
        } else {
            s[j] = total - t;
        }
    }

    println!("{}", s.iter().min().unwrap());
}
