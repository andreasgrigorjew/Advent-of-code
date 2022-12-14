use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inp = fs::read_to_string(&args[1]).expect("Error reading file.");
    let lines = inp.split("\n");
    let oo: usize = 500000;
    
    let mut starts: Vec<(usize, usize)> = vec![];
    let mut end: (usize, usize) = (oo, oo);
    let mut g = vec![];
    for (i, line) in lines.enumerate() {
        g.push(vec![0; line.len()]);
        for (j, mut c) in line.chars().enumerate() {
            if c == 'a' || c == 'S' {
                c = 'a';
                starts.push((i, j));
            } else if c == 'E' {
                c = 'z';
                end = (i, j);
            }
            g[i][j] = c.to_digit(36).unwrap() - 10; // Is there a better way to do this?
        }
    }
    assert!(end != (oo, oo));

    let dx: Vec<i32> = vec![1, 0, -1, 0];
    let dy: Vec<i32> = vec![0, 1, 0, -1];

    let n = g.len() - 1;
    let m = g[0].len();
    let mut adj: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; m]; n];

    let outside = |a, b| a < 0 || a >= n.try_into().unwrap() || b < 0 || b >= m.try_into().unwrap();

    for i_ in 0..n {
        for j_ in 0..m {
            let i: i32 = i_.try_into().unwrap();
            let j: i32 = j_.try_into().unwrap();
            for k in 0..4 {
                if outside(i+dx[k], j+dy[k]) {
                    continue;
                }
                let nx: usize = (i+dx[k]).try_into().unwrap();
                let ny: usize = (j+dy[k]).try_into().unwrap();
                assert!(nx < n);
                assert!(ny < m);
                if g[nx][ny] <= g[i_][j_]+1 {
                    adj[i_][j_].push((nx, ny));
                }
            }
        }
    }

    let mut vis = vec![vec![false; m]; n];
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
    for (a, b) in starts {
        q.push_back((a, b, 0));
        vis[a][b] = true;
    }
    while !q.is_empty() {
        let (a, b, d) = q.pop_front().unwrap();
        assert!(vis[a][b]);

        if (a, b) == end {
            println!("{}", d);
            break;
        }

        for (an, bn) in &adj[a][b] {
            if vis[*an][*bn] {
                continue;
            }
            vis[*an][*bn] = true;
            q.push_back((*an, *bn, d+1));
        }
    }
}
