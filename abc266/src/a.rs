use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let mid = (s.len()+1) / 2;
    println!("{}", s[mid-1]);
}
