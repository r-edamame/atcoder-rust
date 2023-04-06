use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        h: [isize; 3],
        w: [isize; 3],
    }

    let mut count = 0;
    for a11 in 1..=28 {
        for a12 in 1..=28 {
            for a21 in 1..=28 {
                for a22 in 1..=28 {
                    let a13 = h[0] - (a11 + a12);
                    let a23 = h[1] - (a21 + a22);
                    let a31 = w[0] - (a11 + a21);
                    let a32 = w[1] - (a12 + a22);
                    let a33_1 = w[2] - (h[0]-(a11+a12)) - (h[1]-(a21+a22));
                    let a33_2 = h[2] - (w[0]-(a11+a21)) - (w[1]-(a12+a22));

                    if a13 > 0 && a23 > 0 && a31 > 0 && a32 > 0 && a33_1 > 0 && a33_1 == a33_2 {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
