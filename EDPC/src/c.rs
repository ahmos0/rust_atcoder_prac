use proconio::input;

fn main(){
    input!{
        n : usize,
        a : [[usize;3];n],
    }
    let mut dp:[[usize;3]; 100100] = [[0; 3]; 100100];
    let mut tmp1 = 0;
    let mut tmp2 = 0;
    let mut max = 0;
    for i in 0..n{
        for j in 0..3{
            if i == 0{
                dp[i][j] = a[i][j];
            } else {
                if j == 0 {
                    tmp1 = dp[i-1][j+1] + a[i][j];
                    tmp2 = dp[i-1][j+2] + a[i][j];
                    if tmp1 > tmp2{
                        dp[i][j] = tmp1;
                    } else {
                        dp[i][j] = tmp2;
                    }
                } else if j == 1{
                    tmp1 = dp[i-1][j-1] + a[i][j];
                    tmp2 = dp[i-1][j+1] + a[i][j];
                    if tmp1 > tmp2{
                        dp[i][j] = tmp1;
                    } else {
                        dp[i][j] = tmp2;
                    }
                } else if j == 2{
                    tmp1 = dp[i-1][j-2] + a[i][j];
                    tmp2 = dp[i-1][j-1] + a[i][j];
                    if tmp1 > tmp2{
                        dp[i][j] = tmp1;
                    } else {
                        dp[i][j] = tmp2;
                    }
                }
            }
        }
        if i == n - 1{
            for j in 0..3{
                if j == 0{
                    max = dp[n-1][j];
                } else {
                    if max < dp[n-1][j]{
                        max = dp[n-1][j];
                    }
                }
            }
        }
    }
    println!("{}",max);
}