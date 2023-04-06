use proconio::input;
use proconio::marker::{Chars, Usize1, Isize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeMap, BTreeSet};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}

fn btset_singleton(x: isize) -> BTreeSet<isize> {
    let mut s = BTreeSet::new();
    s.insert(x);
    s
}

fn main() {
    input! {
        h: isize,
        w: isize,
        start: (isize, isize),
        n: usize,
        walls: [(isize, isize); n],
        q: usize,
        is: [(char, isize); q]
    }

    let mut horizontal: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();
    let mut vertical: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();

    for (wy, wx) in walls {
        // 縦横の壁情報を作成する
        horizontal.entry(wy)
            .and_modify(|s| { s.insert(wx); })
            .or_insert_with(|| btset_singleton(wx));
        vertical.entry(wx)
            .and_modify(|s| { s.insert(wy); })
            .or_insert_with(|| btset_singleton(wy));
    }

    let mut current = start;
    // for borrow checker
    let wmax = w+1;
    let hmax = h+1;

    for (d, l) in is {
        let &(y, x) = &current;
        match d {
            'U' => {
                let wy = vertical.get(&x).and_then(|s| s.last_lower_than(y)).unwrap_or(&0);
                current.0 = (wy+1).max(y-l);
            }
            'D' => {
                let wy = vertical.get(&x).and_then(|s| s.first_upper_than(y)).unwrap_or(&hmax);
                current.0 = (wy-1).min(y+l);
            }
            'R' => {
                let wx = horizontal.get(&y).and_then(|s| s.first_upper_than(x)).unwrap_or(&wmax);
                current.1 = (wx-1).min(x+l);
            }
            'L' => {
                let wx = horizontal.get(&y).and_then(|s| s.last_lower_than(x)).unwrap_or(&0);
                current.1 = (wx+1).max(x-l);
            }
            _ => panic!()
        }
        println!("{} {}", current.0, current.1);
    }
}
