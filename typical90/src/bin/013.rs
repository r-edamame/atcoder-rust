use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque as Queue, HashSet as HSet, BTreeSet as BSet, BTreeMap as BMap, BinaryHeap as Heap};
use std::ops::{Add, Sub, Mul};
use std::fmt::Display;
use num::pow::Pow;
use std::iter::{FromIterator, IntoIterator};

// BinarySearch
trait BSearch<I> {fn first_upper_than(&self, p: I) -> Option<&I>;fn last_lower_than(&self, p: I) -> Option<&I>;}
impl BSearch<isize> for BSet<isize> {fn first_upper_than(&self, p: isize) -> Option<&isize> {self.range(p+1..).next()} fn last_lower_than(&self, p: isize) -> Option<&isize> {self.range(std::isize::MIN..p).rev().next()}}
impl BSearch<usize> for BSet<usize> {fn first_upper_than(&self, p: usize) -> Option<&usize> {self.range(p+1..).next()} fn last_lower_than(&self, p: usize) -> Option<&usize> {self.range(0..p).rev().next()}}
fn bsearch<F>(lower: isize, upper: isize, sat: F) -> Option<isize>where F: Fn(isize) -> bool{let mut ng = lower-1;let mut ok = upper+1;while (ng-ok).abs() > 1 {let mid = (ng+ok)/2;if sat(mid) {ok = mid;} else {ng = mid;}}return if ok == upper+1 { None } else { Some(ok) };}
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

struct VHeap<T>(Vec<T>);
impl<T: Ord> VHeap<T> {
    fn new() -> VHeap<T> {
        return VHeap(vec![]);
    }
    fn push(&mut self, v: T) {
        self.0.push(v);
        let mut i = self.0.len()-1;
        while i != 0 {
            if self.0[i] > self.0[i/2] {
                self.0.swap(i, i/2);
                i = i/2;
            } else {
                break;
            }
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.0.len() == 0 {
            return None;
        }
        let l = self.0.len();
        self.0.swap(0, l - 1);
        let v = self.0.pop().unwrap();
        let mut i = 0;
        loop {
            if i*2 + 2 >= self.0.len() {
                break;
            }
            if i*2 + 1 == self.0.len() - 1 && self.0[i] < self.0[i*2+1] {
                self.0.swap(i, i*2);
                break;
            }
            if self.0[i] < self.0[i*2+1] || v < self.0[i*2+2] {
                if self.0[i*2 + 1] < self.0[i*2 + 2] {
                    self.0.swap(i, i*2+1);
                    i = i * 2 + 2;
                } else {
                    self.0.swap(i, i*2);
                    i = i * 2 + 1;
                }
            } else {
                break;
            }
        }
        return Some(v);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1, usize); m],
    }

    let mut g = vec![vec![]; n];
    for (from, to, cost) in es {
        g[from].push((to, cost));
        g[to].push((from, cost));
    }

    let front = dijkstra(&g, 0);
    let back = dijkstra(&g, n-1);
    for k in 0..n {
        println!("{}", front[k] + back[k]);
    }
}

fn dijkstra(g: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut dp = vec![0; g.len()];

    let mut used = vec![false; g.len()];
    let mut queue = VHeap::new();
    dp[s] = std::usize::MAX;
    queue.push((std::usize::MAX, s));

    while let Some((_, p)) = queue.pop() {
        if used[p] {
            continue;
        }

        for &(q, c) in g[p].iter() {
            dp[q] = dp[q].max(dp[p] - c);
            queue.push((dp[q], q));
        }
        used[p] = true;
    }
    return dp.iter().map(|&n| std::usize::MAX - n).collect();
}
