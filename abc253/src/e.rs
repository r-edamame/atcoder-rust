use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn dijkstra(to: Vec<Vec<(usize, usize)>>, start: usize) -> Vec<(usize, usize)> {
    let mut shortest = vec![(std::usize::MAX, start); to.len()];
    let mut set = BTreeSet::new();
    shortest[start] = (0, start);
    for (&(next, cost)) in to[start].iter() {
        set.insert((cost, (start, next)));
    }

    while let Some(&p) = set.iter().next() {
        let (total, (from, current)) = p;

        if shortest[current].0 > total {
            shortest[current] = (total, from);
            for &(next, cost) in to[current].iter() {
                if shortest[next].0 > total + cost {
                    set.insert((total+cost, (current, next)));
                }
            }
        }
        set.remove(&p);
    }

    return shortest;
}

fn main() {
    let to: Vec<Vec<(usize, usize)>> = vec![
        // 0
        vec![(1, 5), (2, 1), (3, 4)],
        // 1
        vec![(0, 5), (4, 2)],
        // 2
        vec![(0, 1), (3, 2)],
        // 3
        vec![(0, 4), (2, 2), (4, 5), (5, 6)],
        // 4
        vec![(1, 2), (3, 5), (6, 1), (7, 3)],
        // 5
        vec![(3, 6), (7, 2)],
        // 6
        vec![(4, 1), (7, 4)],
        // 7
        vec![(4, 3), (5, 2), (6, 4)],
    ];

    let shortest = dijkstra(to, 0);

    let mut current = 7;
    loop {
        println!("{}", current);
        current = shortest[current].1;
        if current == 0 {
            println!("{}", current);
            break;
        }
    }
}
