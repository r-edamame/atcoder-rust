use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet, HashMap};


fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut file_count: HashMap<String, usize> = HashMap::new();

    for s in ss {
        if let Some(&c) = file_count.get(&s) {
            println!("{}", [s.clone(), "(".to_string(), c.to_string(), ")".to_string()].concat());
            file_count.insert(s, c+1);
        } else {
            println!("{}", s);
            file_count.insert(s, 1);
        }
    }
}
