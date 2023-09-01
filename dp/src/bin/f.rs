use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        s: String,
        t: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 3100]; 3100];
    for i in 0..s_c.len(){
        for j in 0..t_c.len(){
            if s_c[i] == t_c[j]{
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j]+1);
            }
            dp[i+1][j+1] = max(dp[i+1][j+1], dp[i+1][j]);
            dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j+1]);
        }
    }
    let mut i = s.len();
    let mut j = t.len();
    let mut res = String::new();

    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            let c = s_c[i-1];
            res.insert(0, c);
            i -= 1;
            j -= 1;
        }
    }

    println!("{}", res);

}
