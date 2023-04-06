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
        map: [Chars; 9],
    }

    let valid = |(x, y): (isize, isize)| { (0..9).contains(&x) && (0..9).contains(&y) && map[y as usize][x as usize] == '#' };

    let mut squares: HashSet<Vec<(isize, isize)>> = HashSet::new();

    for x in 0..=9 {
        for y in 0..=9 {

            for sx in -8..=8 {
                for sy in -8..=8 {
                    if sx == 0 && sy == 0 {
                        continue;
                    }

                    let p2 = (x+sx, y+sy);
                    let p3 = (x+sx-sy, y+sy+sx);
                    let p4 = (x-sy, y+sx);
                    if valid((x, y)) && valid(p2) && valid(p3) && valid(p4) {
                        let mut sqr = BTreeSet::new();
                        sqr.insert((x,y));
                        sqr.insert(p2);
                        sqr.insert(p3);
                        sqr.insert(p4);
                        squares.insert(sqr.into_iter().collect());
                    }
                }
            }
        }
    }

    println!("{}", squares.len());
}
