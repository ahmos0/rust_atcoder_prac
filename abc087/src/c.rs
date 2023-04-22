use proconio::input;

fn main() {
    input! {
        n : usize,
        arr : [[i64; n]; 2],
    }
    let mut dp: [[i64; 110]; 2] = [[0; 110]; 2];
    for i in 0..2 {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[0][0] = arr[0][0];
            } else if i == 0 {
                dp[0][j] = arr[0][j] + dp[0][j - 1];
            } else if i == 1 {
                if j == 0 {
                    dp[i][j] = dp[0][0] + arr[1][0];
                } else {
                    let a = arr[1][j] + dp[0][j];
                    let b = arr[1][j] + dp[1][j - 1];
                    if a > b {
                        dp[i][j] = a;
                    } else {
                        dp[i][j] = b;
                    }
                }
            }
        }
    }
    println!("{}", dp[1][n - 1]);
}
