use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        n: usize,
        w: i128,
        p: [(i128, i128); n]
    }
    let mut dp: Vec<Vec<i128>> = vec![vec![9999999999; 100010]; 110];
    dp[0][0] = 0;
    for i in 0..n{
        for sum_v in 0..100010{
            if sum_v - p[i].1 >= 0{
                dp[i+1][sum_v as usize] = min(dp[i+1][sum_v as usize], dp[i][(sum_v as usize) - (p[i].1 as usize)] + p[i].0);
            }
            dp[i+1][sum_v as usize] = min(dp[i+1][sum_v as usize], dp[i][sum_v as usize]);
        }
    }
    let mut result = 0;
    for sum_v in 0..100010{
        if dp[n][sum_v] <= w{
            result = sum_v;
        }
    }
    println!("{}", result);
}
