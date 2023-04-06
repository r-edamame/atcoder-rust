use proconio::input;

const X: i64 = 998244353;

fn main() {
    input! {
        n: i64,
    }

    let x = n.rem_euclid(X);
    println!("{}", x);
}