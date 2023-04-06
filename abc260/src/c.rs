use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut red = vec![0; 12];
    let mut blue = vec![0; 12];
    blue[1] = 1;

    for i in 2..=n {
        blue[i] = red[i-1] + blue[i-1] * y;
        red[i] = red[i-1] + blue[i]*x;
    }

    println!("red {:?}", red);
    println!("blue {:?}", blue);
    println!("{}", red[n]);
}
