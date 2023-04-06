use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::{VecDeque, HashSet};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point { x: i64, y: i64 }

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let out_of_bound = move |pos: &Point, dir: char| pos.x==0 && dir=='L' || pos.x==(w as i64)-1&&dir=='R' || pos.y==0&&dir=='U' || pos.y==(h as i64)-1&&dir=='D';

    let mut passed: HashSet<Point> = HashSet::new();
    let mut pos = Point { x: 0, y: 0 };
    passed.insert(pos.clone());
    loop {
        let dir: char = grid[pos.y as usize][pos.x as usize];
        if out_of_bound(&pos, dir) {
            println!("{} {}", pos.y+1, pos.x+1);
            break;
        }

        match dir {
            'U' => { pos.y -= 1 },
            'D' => { pos.y += 1 },
            'L' => { pos.x -= 1 },
            'R' => { pos.x += 1 },
            _ => panic!()
        }

        if !passed.insert(pos.clone()) {
            println!("-1");
            break;
        }
    }
}
