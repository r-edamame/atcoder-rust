use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::hash_map::RandomState;
use std::collections::{VecDeque, HashSet, HashMap};
use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher;

type Hasher = BuildHasherDefault<FxHasher>;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        ws: [(i64, i64); 3],
        obs: [(i64, i64); m],
    }

    let mut dp: HashMap<(i64, i64), i64> = HashMap::default();
    dp.insert((0,0), 1); 
    let mut obstructs: HashSet<(i64, i64), Hasher> = HashSet::default();
    for o in obs {
        obstructs.insert(o);
    }
    println!("{}", calc(dp, &ws, &obstructs, n));
}


fn calc(_dp: HashMap<(i64, i64), i64>, pattern: &Vec<(i64, i64)>, obstructs: &HashSet<(i64, i64), Hasher>, mut n: usize) -> i64 {
    // println!("n: {}", n);
    let mut dp: HashMap<(i64, i64), i64, Hasher> = HashMap::default();
    dp.insert((0,0), 1);


    while n > 0 {
        // let mut new_dp: HashMap<(i64, i64), i64> = HashMap::default();
        let mut new_dp: HashMap<(i64, i64), i64, Hasher> = HashMap::with_hasher(Hasher::default());
        for &(x, y) in dp.keys() {
            for (px, py) in pattern {
                if !obstructs.contains(&(x+px, y+py)) {
                    let from = dp.get(&(x, y)).unwrap();
                    new_dp.entry((x+px, y+py))
                        .and_modify(|c| { *c = (*c + from) % MOD; })
                        .or_insert(*from);
                }
            }
        }
        dp = new_dp;
        n -= 1;
        println!("dp: {}", dp.len());
    }


    let mut sum = 0;
    dp.into_values().for_each(|v| {
        sum = (sum + v) % MOD;
    });
    return sum;

    // return calc(new_dp, pattern, obstructs, n-1);
}
