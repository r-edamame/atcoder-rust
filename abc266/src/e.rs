use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let e = solve(n);
    println!("{}", e);
}

fn solve(n: i32) -> f64 {
    let un: usize = n as usize;
    let mut dp = vec![0f64; un+1];

    for i in 1..=un {
        dp[i] = (1..=6).map(|x| (x as f64).max(dp[i-1]) / 6f64).sum();
    }
    return dp[un];
}
