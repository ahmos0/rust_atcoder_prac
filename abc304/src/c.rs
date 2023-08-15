
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        d: i32,
        b: [(i32, i32); n]
    }

    let mut graph: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if (b[i].0 - b[j].0).pow(2) + (b[i].1 - b[j].1).pow(2) <= d.pow(2) {
                graph[i][j] = true;
            }
        }
    }

    let mut ans: Vec<bool> = vec![false; n];
    ans[0] = true;
    let mut que = VecDeque::new();
    que.push_back(0);

    while let Some(q) = que.pop_front() {
        for i in 0..n {
            if graph[q][i] && !ans[i] {
                ans[i] = true;
                que.push_back(i);
            }
        }
    }

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
