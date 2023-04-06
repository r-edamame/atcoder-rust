use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::{Ordering, Reverse};
use std::collections::{VecDeque, HashSet};


fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut bss: Vec<Vec<usize>> = vec![Vec::with_capacity(n/k + 1); k];

    for i in 0..n {
        bss[i%k].push(a[i]);
    }

    bss.iter_mut().for_each(|bs| bs.sort_by_key(|k| Reverse(*k)));

    a.sort();

    for i in 0..n {
        if a[i] != bss[i%k].pop().unwrap() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
