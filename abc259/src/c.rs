use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        s: Chars,
        t: Chars
    }


    let ns = comp(s);
    let nt = comp(t);

    if ns.len() == nt.len() {
        for (&(cs, ls), (ct, lt)) in ns.iter().zip(nt) {
            if !(cs == ct && (ls == lt || ls <= lt && ls >= 2)) {
                println!("No");
                return;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}

fn comp(s: Vec<char>) -> Vec<(char, usize)> {
    let mut current = (s[0], 1);
    let mut res = vec![];

    for i in 1..s.len() {
        if current.0 != s[i] {
            res.push(current);

            current = (s[i], 1);
        } else {
            current.1 += 1;
        }
    }
    res.push(current);

    return res;
}