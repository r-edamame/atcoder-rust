use proconio::input;
use proconio::marker::{Usize1};
use std::collections::{VecDeque, HashSet};

fn main() {
    // input! {
    //     n: usize,
    //     es: [(Usize1, Usize1); n],
    //     q: i32,
    //     qs: [(Usize1, Usize1); q],
    // };

    let n = 10000;
    let q = 10;
    let (es, qs) = create_testcase(n, q);

    println!("{}", solve(n, es, qs).iter().map(|&b| if b { "Yes" } else { "No" }).collect::<Vec<&str>>().join("\n"));
}

fn create_testcase(n: usize, q: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut es: Vec<(usize, usize)> = Vec::with_capacity(n);
    let mut chose: Vec<usize> = Vec::with_capacity(n);

    let circle_points_num = (rand::prelude::random::<usize>()%(n-2-3))+3;

    let mut circle: Vec<usize> = Vec::with_capacity(circle_points_num);
    for _ in 0..circle_points_num {
        loop {
            let p = rand::prelude::random::<usize>() % n;
            if !circle.contains(&p) {
                circle.push(p);
                break;
            }
        }
    }
    println!("circle generated");

    for i in 0..circle.len() {
        let pair: (usize, usize) = (circle[i], circle[(i+1)%circle.len()]);
        es.push(pair);
        chose.push(circle[i]);
    }


    (0..n).filter(|x| !circle.contains(x))
        .for_each(|x| {
            loop {
                let p = rand::prelude::random::<usize>() % n;
                if chose.contains(&p) {
                    es.push((x, p));
                    chose.push(x);
                    println!("chose: {}", chose.len());
                    break;
                }
            }
        });
    println!("pairs generated");

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut qs: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..q {
        loop {
            let x = rand::prelude::random::<usize>() % n;
            let y = rand::prelude::random::<usize>() % n;

            if x != y && set.insert((x.min(y), x.max(y))) {
                qs.push((x, y));
                break;
            }
        }
    }

    println!("testcase generated!");
    println!("es: {:?}\nqs: {:?}", es, qs);
    return (es, qs);
}

fn solve(n: usize, es: Vec<(usize, usize)>, qs: Vec<(usize, usize)>) -> Vec<bool> {
    let mut degree = vec![0; n];

    let mut to: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];

    for (u, v) in es {
        to[u].push(v);
        to[v].push(u);

        degree[u] += 1;
        degree[v] += 1;
    }

    let mut in_cycle: Vec<bool> = vec![true; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    (0..n).filter(|&i| degree[i] == 1).for_each(|i| queue.push_back(i));
    while let Some(p) = queue.pop_front() {
        in_cycle[p] = false;
        degree[p] -= 1;

        for &e in &to[p] {
            if degree[e] > 1 {
                degree[e] -= 1;
                if degree[e] == 1 {
                    queue.push_back(e);
                }
            }
        }
    }
    println!("in_cycle: {:?}", in_cycle);

    let mut group: Vec<usize> = vec![n; n] ;

    for i in 0..n {
        if group[i] == n {
            if !in_cycle[i] {
                continue;
            }

            group[i] = i;

            let mut q2: VecDeque<usize> = VecDeque::new();
            to[i].iter().filter(|&&x| !in_cycle[x]).for_each(|&x| q2.push_back(x));
            while let Some(p) = q2.pop_front() {
                group[p] = i;
                to[p].iter().filter(|&&x| group[x] == n).for_each(|&x| q2.push_back(x));
            }
        }
    }
    println!("group: {:?}", group);

    return qs.iter().map(|&(x, y)| group[x] == group[y]).collect();
}
