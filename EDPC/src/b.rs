// 解けてない
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h : [i64; n],
    }
    let mut dp: [i64; 100010] = [0; 100010];
    for i in 0..n {
        if i == 0 {
            dp[i] = 0;
        } else if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs();
        } else {
            let mut b = 100010;
            let mut a = 0;
            if k == 1 {
                a = (h[i] - h[i - 1]).abs() + dp[i - 1];
                if a < b {
                    b = a;
                }
            } else {
                for j in 1..=k {
                    if i < j {
                        break;
                    } else if i == 2 {
                        a = (h[i - 2] - h[i]).abs();
                    } else {
                        a = (h[i - j] - h[i]).abs() + dp[i - j];
                    }
                    if a < b {
                        b = a;
                    }
                }
            }

            dp[i] = b;
        }
    }
    println!("{}", dp[n - 1]);
}
