use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let mut odds: Vec<usize> = vec![];
    let mut evens: Vec<usize> = vec![];
    for a in ai {
        if a&1 ==0 {
            evens.push(a);
        } else {
            odds.push(a);
        }
    }

    odds.sort();
    odds.reverse();
    evens.sort();
    evens.reverse();

    let oo = odds.get(0).and_then(|x| odds.get(1).map(|y| x + y));
    let ee = evens.get(0).and_then(|x| evens.get(1).map(|y| x + y));

    println!("{}", match (oo, ee) {
        (Some(oov), Some(eev)) => oov.max(eev) as i64,
        (Some(oov), _) => oov as i64,
        (_, Some(eev)) => eev as i64,
        _ => -1
    });
}
