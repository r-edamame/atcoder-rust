use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

type Count = usize;

fn main() {
    input! {
        q: usize,
    }

    let mut cyl: VecDeque<(usize, Count)> = VecDeque::with_capacity(200000);

    for i in 0..q {
        input!{
            command: usize,
        }
        if command == 1 {
            input! {
                x: usize,
                c: usize,
            }
            cyl.push_back((x, c));
        } else {
            let mut sum = 0;
            input! {
                mut c: usize,
            }
            while c > 0 {
                let (x, cnt) = cyl.pop_front().unwrap();
                if c >= cnt {
                    c -= cnt;
                    sum += x * cnt;
                } else {
                    sum += x * c;
                    cyl.push_front((x, cnt-c));
                    c = 0;
                }
            }
            println!("{}", sum);
        }
    }
}
