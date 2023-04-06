use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque as Queue, HashSet as HSet, BTreeSet as BSet, BTreeMap as BMap};
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


fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut mat = vec![vec![false; w]; h];
    let ix = |x: usize, y: usize| { y * w + x };
    let mut uf = UnionFind::new(w * h);

    for i in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    y: Usize1,
                    x: Usize1,
                }

                mat[y][x] = true;

                if x != 0 && mat[y][x-1] {
                    uf.union(ix(x, y), ix(x-1, y));
                }
                if x != w - 1 && mat[y][x+1] {
                    uf.union(ix(x, y), ix(x+1, y));
                }
                if y != 0 && mat[y-1][x] {
                    uf.union(ix(x, y), ix(x, y-1));
                }
                if y != h - 1 && mat[y+1][x] {
                    uf.union(ix(x, y), ix(x, y+1));
                }
            },
            2 => {
                input! {
                    fy: Usize1,
                    fx: Usize1,
                    ty: Usize1,
                    tx: Usize1,
                }

                yesno(mat[fy][fx] && mat[ty][tx] && uf.same(ix(fx, fy), ix(tx, ty)));
            },
            _ => panic!()
        }
    }
}
