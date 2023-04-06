use proconio::input;
use std::ops::{Mul, Sub};
use std::cmp::PartialOrd;
use num::{Zero, Signed};

fn main() {
    input! {
        mut points: [(i32, i32); 4],
    }

    let convex = is_convex::<i32>(&points);
    println!("{}", if convex {"Yes"} else {"No"});
}

fn is_convex<N: Mul<Output = N> + Sub<Output = N> + PartialOrd + Zero + Signed + Copy>(points: &Vec<(N, N)>) -> bool {
    let to_v = |&(px, py), &(qx, qy)| { (qx-px, qy-py) };
    let outer = |(px, py), (qx, qy)| { px*qy - py*qx };
    let len = points.len();

    if len <= 2 {
        return false;
    }

    let outer_sigs: Vec<N> = (0..len)
        .map(|i| {
            let n1 = if i+1<len {i+1} else {i+1-len};
            let n2 = if i+2<len {i+2} else {i+2-len};
            let v1 = to_v(&points[i], &points[n1]);
            let v2 = to_v(&points[n1], &points[n2]);

            outer(v1, v2).signum()
        }).collect();

    let sat = outer_sigs.iter().all(|&s| s > N::zero()) || outer_sigs.iter().all(|&s| s < N::zero());
    return sat;
}
