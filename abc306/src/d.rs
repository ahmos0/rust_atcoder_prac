use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n:usize,
        data: [(usize, i128); n]
    }
    let mut dp: Vec<Vec<i128>> = vec![vec![0; 2]; 300010];
    for i in 0..n {
        let (a, b) = data[i];
        if i == 0 {
            if a == 0 {
                dp[0][0] = b;
            } else {
                dp[0][1] = b;
            }
        } else {
            if a == 0 {
                dp[i + 1][0] = max(dp[i][0], max(dp[i][0] + b, dp[i][1] + b));
            } else {
                dp[i + 1][1] = max(dp[i][1], dp[i][0] + b);
            }
        }
        dp[i + 1][0] = max(dp[i + 1][0], dp[i][0]);
        dp[i + 1][1] = max(dp[i + 1][1], dp[i][1]);
    }
    println!("{}", max(dp[n][0], dp[n][1]));
}
