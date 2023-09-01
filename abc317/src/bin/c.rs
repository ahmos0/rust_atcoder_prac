use proconio::{input, marker::Usize1};
use std::cmp;

fn main() {
    input! {
        n: usize, 
        m: usize, 
        roads: [(Usize1, Usize1, u32); m] 
    }

    let mut adjacency_matrix = vec![vec![0; n]; n];
    for &(from, to, length) in &roads {
        adjacency_matrix[from][to] = length;
        adjacency_matrix[to][from] = length; 
    }

    let mut dp = vec![vec![-1; 1 << n]; n];

    let mut max_distance = 0;

    for start in 0..n {
        max_distance = cmp::max(max_distance, dfs(start, 1 << start, &adjacency_matrix, &mut dp));
    }

    println!("{}", max_distance);
}

fn dfs(
    current: usize,
    visited: usize,
    adjacency_matrix: &Vec<Vec<u32>>,
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    let n = adjacency_matrix.len();
    if visited == (1 << n) - 1 {
        return 0;
    }

    if dp[current][visited] != -1 {
        return dp[current][visited];
    }

    let mut max_distance = 0;
    for next in 0..n {
        if (visited >> next) & 1 == 0 && adjacency_matrix[current][next] > 0 {
            max_distance = cmp::max(
                max_distance,
                dfs(next, visited | (1 << next), adjacency_matrix, dp)
                    + adjacency_matrix[current][next] as i32,
            );
        }
    }

    dp[current][visited] = max_distance;
    max_distance
}
