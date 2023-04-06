use std::time::Instant;
use std::collections::{BTreeSet, HashMap};



fn main() {
    let mut set = BTreeSet::new();

    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);

    let mut set2 = set.split_off(&4);

    println!("{:?}", set);
    println!("{:?}", set2);
}