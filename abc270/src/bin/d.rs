fn main(){proconio::input!{n:usize,k:usize,v:[usize;k],}let mut d=vec![0;n+1];for i in 1..=n{d[i]=v.iter().filter_map(|&c|if c>i{None}else{Some(c+(i-c)-d[i-c])}).max().unwrap();}println!("{}",d[n]);}
