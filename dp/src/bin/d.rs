use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: i128,
        p: [(i128, i128); n]
    }
    let mut dp: Vec<Vec<i128>> = vec![vec![0; (w+1) as usize]; n+1];
    for i in 0..n {
        for sum_w in 0..=w {
            if sum_w as i128 - p[i as usize].0 >= 0 {
                
                
                dp[i+1][sum_w as usize] = max(dp[i+1][sum_w as usize], dp[i][(sum_w as usize) - (p[i].0 as usize)] + p[i].1);
            }
            dp[i+1][sum_w as usize] = max(dp[i+1][sum_w as usize], dp[i][sum_w as usize]);
        }
    }
    println!("{}",dp[n][w as usize]);
}
