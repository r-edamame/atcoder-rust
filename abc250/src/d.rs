use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, BTreeSet};

trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BTreeSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BTreeSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}

fn lower_bound<F>(lower: isize, upper: isize, sat: F) -> Option<isize> where F: Fn(isize) -> bool {
    let mut ok = lower-1;
    let mut ng = upper+1;

    while (ok-ng).abs() > 1 {
        let mid = (ok+ng)/2;
        if sat(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    return if ok == lower-1 { None } else { Some(ok) };
}

fn main() {
    input! {
        n: usize,
    }

    let primes = eratosthenes(1000000);

    let mut count: usize = 0;

    for (ix, &p) in primes.iter().enumerate() {
        let np = (p-1).min(n / p.pow(3));
        if let Some(i) = lower_bound(0, ix as isize-1, |i| primes[i as usize] <= np) {
            count += (i as usize)+1
        }
    }

    println!("{:?}", count);
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n+1];

    sieve[1] = false;
    for i in 2..=n/2 {
        sieve[i*2] = false;
    }

    for i in (3..((n as f64).sqrt() as usize)).step_by(2) {
        for j in 2..=n/i {
            sieve[j*i] = false;
        }
    }

    let mut primes = Vec::new();
    for i in 1..=n {
        if sieve[i] {
            primes.push(i);
        }
    }

    return primes;
}
