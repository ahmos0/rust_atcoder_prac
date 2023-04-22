use proconio::input;

fn main(){
    input!{
        N : usize,
        h : [i64; N],
    }
    let mut dp : [i64; 100010] = [0; 100010];
    for i in 0..N{
        if i == 0{
            dp[i] = h[i];
        } else if i == 1{
            dp[i] = (h[i] - h[i-1]).abs();
        } else {
            let a = (h[i-1] - h[i]).abs() + dp[i-1];
            let mut b = 0;
            if i == 2{
                b = (h[i-2] - h[i]).abs();
            } else {
                b = (h[i-2] - h[i]).abs() + dp[i-2];
            }
            
            if a > b{
                dp[i] = b;
            } else{
                dp[i] = a;
            }
        }
    }
    println!("{}",dp[N-1]);
}   