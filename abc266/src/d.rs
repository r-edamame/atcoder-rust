use proconio::input;
use std::cmp;

fn main () {
    input! {
        n: i32,
        mut r: [(i64, i64, i64); n],
    }

    println!("{}", solve(&mut r));
}

fn solve(r: &mut Vec<(i64, i64, i64)>) -> i64 {
    let latest = r[r.len()-1].0;
    let ix = |t, p| { (t*5 + p) as usize };
    let mut dp: Vec<i64> = vec![0; ((latest+1)*5) as usize];
    let mut i: usize = 0;

    dp[0] = 0;
    dp[1] = -1;
    dp[2] = -1;
    dp[3] = -1;
    dp[4] = -1;
    for time in 1..latest+1 {
        for point in 0..5 {
            let cur = ix(time.clone(), point.clone());
            let from_left = if point == 0 { -1 } else { dp[cur-6] };
            let from_right = if point == 4 { -1 } else { dp[cur-4] };
            let from_center = dp[cur-5];

            dp[cur] = cmp::max(from_left, cmp::max(from_right, from_center));
        }

        if r[i].0 == time {
            let (_t, x, a) = r[i];
            let target = ix(time, x);
            if dp[target] >= 0 {
                dp[target] += a;
            }
            i += 1;
        }
    }

    return *dp.iter().max().unwrap();
}