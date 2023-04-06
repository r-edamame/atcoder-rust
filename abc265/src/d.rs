use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};

fn bsearch<F>(s: usize, e: usize, p: F) -> Option<usize>
where F: Fn(usize) -> Ordering {
    if s > e {
        return None;
    }
    let mid = (s+e)/2;
    match p(mid) {
        Ordering::Equal => Some(mid),
        Ordering::Less => if mid == s { None } else { bsearch(s, mid, p) },
        Ordering::Greater => bsearch(mid+1, e, p)
    }
}

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        ai: [usize; n],
    }

    let mut s = vec![0; n+1];
    for i in 1..n+1 {
        s[i] += s[i-1] + ai[i-1];
    }

    for x in 0..=n-3 {
        let res = bsearch(x+1, s.len()-2, |y| { p.cmp(&(s[y]-s[x])) })
            .and_then(|y| bsearch(y+1, s.len()-1, |z| { q.cmp(&(s[z]-s[y])) }))
            .and_then(|z| bsearch(z+1, s.len(), |w| { r.cmp(&(s[w]-s[z])) }));
        
        if res.is_some() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
