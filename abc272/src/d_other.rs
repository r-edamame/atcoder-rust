use proconio::input;
use std::collections::VecDeque;
 
fn main() {
    input!{
        n: usize,
        m: usize,
    };
    let mut dist = vec![vec![-1; n]; n];
    dist[0][0] = 0;
 
    let mut dir = vec![];
    for i in 0..n {
        for j in 0..n {
            if i * i + j * j == m {
                dir.push((i, j));
            }
        }
    }
 
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((ux, uy)) = que.pop_front() {
        for &(dx, dy) in dir.iter() {
            // if dx * dx + dy * dy != m {continue;}
 
            if ux + dx < n && uy + dy < n {
                let (px, py) = (ux + dx, uy + dy);
                if dist[px][py] == -1 {
                    dist[px][py] = dist[ux][uy] + 1;
                    que.push_back((px, py));
                }
            }
            if ux + dx < n && dy <= uy {
                let (px, py) = (ux + dx, uy - dy);
                if dist[px][py] == -1 {
                    dist[px][py] = dist[ux][uy] + 1;
                    que.push_back((px, py));
                }
            }
 
            if dx <= ux && uy + dy < n {
                let (px, py) = (ux - dx, uy + dy);
                if dist[px][py] == -1 {
                    dist[px][py] = dist[ux][uy] + 1;
                    que.push_back((px, py));
                }
            }
 
            if dx <= ux && dy <= uy {
                let (px, py) = (ux - dx, uy - dy);
                if dist[px][py] == -1 {
                    dist[px][py] = dist[ux][uy] + 1;
                    que.push_back((px, py));
                }
            }
        }
    }
 
    for i in 0..n {
        for j in 0..n {
            print!("{} ", dist[i][j]);
        }
        println!();
    }
}
