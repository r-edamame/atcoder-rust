use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::{thread, time};

const MAX_N: i64 = 400;
const MAX_M: i64 = 1000000;

fn main() {
    input! {
        n: i64,
        m: i64,
    }

    // まず、各点からどこに進めるのかを調べる
    let mut move_pattern: Vec<(i64, i64)> = vec![];
    for x in -400..400 {
        for y in -400..400 {
            if m == (x*x) + (y*y) {
                move_pattern.push((x, y));
            }
        }
    }

    // println!("move_pattern: {:?}", move_pattern);

    let mut step_to_reach: Vec<Vec<i32>> = vec![vec![-1; n as usize]; n as usize];
    step_to_reach[0][0] = 0;

    // 各点において、進める方向マスをキューに入れる
    // 既に通過している点であれば何もしない
    let mut passed: HashSet<(i64, i64)> = HashSet::new();
    passed.insert((0,0));
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back((0,0));
    while let Some((x, y)) = queue.pop_front() {

        for &(px, py) in move_pattern.iter() {
            let (nx, ny) = (x+px, y+py);
            // 場外に出たらスキップ
            if nx < 0 || nx >= n || ny < 0 || ny >= n {
                continue;
            }

            if passed.insert((nx, ny)) {
                queue.push_back((nx, ny));
                step_to_reach[ny as usize][nx as usize] = step_to_reach[y as usize][x as usize] + 1;
            }
        }
    }

    print_mat(&step_to_reach);
}

fn print_mat(mat: &Vec<Vec<i32>>) {
    println!("{}", mat.iter()
        .map(|ss| ss.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "))
        .collect::<Vec<String>>().join("\n")
    );
}