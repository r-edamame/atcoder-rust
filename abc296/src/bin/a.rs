use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque as Queue, HashSet as HSet, BTreeSet as BSet, BTreeMap as BMap};
use std::ops::{Add, Sub, Mul};
use std::fmt::Display;
use num::pow::Pow;
use std::iter::{FromIterator, IntoIterator};
use itertools::Itertools;
use rand::Rng;

// BinarySearch
trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn upper_bound<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
fn lower_bound<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ok = lower-1;let mut ng = upper+1;while (ok-ng).abs() > 1 {let mid = (ok+ng)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == lower - 1 { None } else { Some(ok) };}
fn nearest<V: Copy + PartialOrd + Sub<Output = V>>(v: &Vec<V>, t: V) -> V {let mut cs = vec![];if let Some(ii) = upper_bound(0, v.len() as isize - 1, |i| t < v[i as usize]) {let i = ii as usize;if i == 0 {cs.push(v[i]);cs.push(v[v.len() - 1]);} else {cs.push(v[i]);cs.push(v[i - 1])}} else {cs.push(v[0]);cs.push(v[v.len() - 1]);}return *cs.iter().min_by(|&&a, &&b| {abs_diff(t, a).partial_cmp(&abs_diff(t, b)).unwrap()}).unwrap();}
// cumulative sum
fn cumlative<N: std::ops::Add + num::Zero + Clone + Copy>(v: Vec<N>) -> Vec<N> {let mut a: Vec<N> = vec![N::zero(); v.len()+1];for i in 0..v.len() {a[i+1] = a[i] + v[i]}a}
// Mint
const MINT_MOD: isize = 998244353;
#[derive(Clone, Copy)]
struct Mint(isize);impl Add for Mint {type Output = Mint;fn add(self, rhs: Self) -> Self::Output {Mint(((self.0 % MINT_MOD) + (rhs.0 % MINT_MOD)) % MINT_MOD)}}impl Sub for Mint {type Output = Mint;fn sub(self, rhs: Self) -> Self::Output {Mint(((self.0 % MINT_MOD) - (rhs.0 % MINT_MOD)).rem_euclid(MINT_MOD))}}impl Mul for Mint {type Output = Mint;fn mul(self, rhs: Self) -> Self::Output {Mint(((self.0 % MINT_MOD) * (rhs.0 % MINT_MOD)) % MINT_MOD)}}impl Display for Mint {fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {f.write_fmt(format_args!("{}", self.0))}}impl Pow<usize> for Mint {type Output = Mint;fn pow(self, rhs: usize) -> Self::Output {match rhs {0 => Mint(1),1 => self,n => {let m = self.pow(n/2);if n&1==0 { m*m } else { self * m*m }}}}}
// UnionFind
struct UnionFind{roots: Vec<usize>, ranks: Vec<usize>}
impl UnionFind {fn new(size: usize) -> Self {UnionFind { roots: (0..size).collect(), ranks: vec![0; size] }}fn root(&mut self, x: usize) -> usize {if self.roots[x] == x {return x;}let r = self.root(self.roots[x]);self.roots[x] = r;return r;}fn same(&mut self, x: usize, y: usize) -> bool {self.root(x) == self.root(y)}fn union(&mut self, x: usize, y: usize) {let rx = self.root(x);let ry = self.root(y);if rx != ry {if self.ranks[rx] < self.ranks[ry] {self.roots[rx] = ry;} else {self.roots[ry] = rx;if self.ranks[x] == self.ranks[y] {self.ranks[x] += 1;}}}}}
// fmt
fn yesno(b: bool) {println!("{}", if b { "Yes" } else { "No" });}
// bit
fn gen_iter<I: Copy + PartialOrd, F>(first: I, to: I, f: &'static F) -> impl Iterator<Item=I>where F: Fn(I) -> I{std::iter::successors(Some(first), move |x| if f(*x) >= to { None } else { Some(f(*x)) })}
fn bit_digits(to: usize) -> impl Iterator<Item=usize> {gen_iter(1, to, &|x| x << 1)}
fn bit_choose<T: Copy>(v: &Vec<T>) -> impl Iterator<Item = Vec<T>> + '_ {(0usize..1<<v.len()).map(move |n| (0usize..v.len()).filter_map(|i| if n & 1usize<<i > 0 { Some(v[i]) } else { None }).collect::<Vec<T>>())}

fn abs_diff<V: Copy + PartialOrd + Sub<Output = V>>(a :V, b: V) -> V {(if a > b { a } else { b }) - (if a < b { a } else { b })}
fn udg(v: Vec<(usize, usize)>, n: usize) -> Vec<Vec<usize>> {let mut g = vec![vec![]; n];for (t, f) in v {g[t].push(f);g[f].push(t);}g}

// [    8                 ]
// [    4     ]
// [ 2  ]      [   6]
// [1]   [3]   [5]   [7]   

// add 1 x -> t[1], t[2], t[4], t[8] += x  | 0001 0010 0100 1000
// add 2 x -> t[2], t[4], t[8] += x        | 0010 0100 1000
// add 3 x -> t[3], t[4], t[8] += x        | 0011 0100 1000
// add 4 x -> t[4], t[8] += x              | 0100 1000
// add 5 x -> t[5], t[6], t[8] += x        | 0101 0110 1000
// add 6 x -> t[6], t[8] += x              | 0110 1000
// add 7 x -> t[7], t[8] += x              | 0111 1000

// sum 1 -> 0001
// sum 2 -> 0010
// sum 3 -> 0011 0010
// sum 4 -> 0100
// sum 5 -> 0101 0100
// sum 6 -> 0110 0100
// sum 7 -> 0111 0110 0100
// sum 8 -> 1000

struct BIT(Vec<usize>);
impl BIT {
    fn new(n: usize) -> BIT {
        BIT(vec![0; n+1])
    }
    fn add(&mut self, mut i: usize, v: usize) {
        // 0 origin -> 1 origin
        i += 1;
        while i <= self.0.len() {
            let b = (-(i as isize) & (i as isize)) as usize;
            self.0[i] += v;
            i += b
        }
    }
    fn sum(&self, mut t: usize) -> usize {
        t += 1;
        let mut sum = 0;
        while t != 0 {
            let b = (-(t as isize) & (t as isize)) as usize;
            sum += self.0[t];
            t -= b;
        }
        return sum
    }
}

fn main() {
    input! {
    }

    let mut rng = rand::thread_rng();

    let mut vs = vec![];
    for _ in 0..200000 {
        vs.push((rng.gen::<usize>() % 1000) + 1);
    }

    let mut bit = BIT::new(1001);

    let mut ans = 0;
    for i in 0..vs.len() {
        ans += bit.sum(1000) - bit.sum(i-1);
        bit.add(i, 1);
    }

    println!("{}", ans);
}
