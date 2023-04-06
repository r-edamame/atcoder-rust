use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }


    let rad = d / 180.0 * std::f64::consts::PI;
    let nx = a * rad.cos() - b * rad.sin();
    let ny = a * rad.sin() + b * rad.cos();

    println!("{} {}", nx, ny);
}
