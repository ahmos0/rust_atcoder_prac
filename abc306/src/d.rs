use proconio::input;

fn main(){
    input!{
        n:usize,
        data: [(usize, i128); n]
    }
    let mut judge = false;
    let mut dp_judge : [bool; 300010] = [false; 300010];
    let mut dp : [i128; 300010] = [0; 300010];
    for i in 0..n{
        let (a, b) = data[i];
        if a == 1 {
            judge = true;
        } else {
            judge = false;
        } 
        if i == 0{
            dp[i] = b;
            dp_judge[i] = judge;
        } else if i == 1{
            if judge == true  && dp[i-1] < b {
                dp[i] = b;
            } else if judge == false  && 0 < b {
                dp[i] = dp[i-1] + b;
                dp_judge[i] = judge;
            } else if judge == true && 0 < b{
                dp[i] = dp[i-1] + b;
                dp_judge[i] = judge;
            }
        } else {
            if judge == false{
                if dp[i-1] > dp[i-2]{
                    dp[i] = dp[i-1]+b;
                } else{
                    dp[i] = dp[i-2]+b;
                }
                dp_judge[i] = judge;
            } else if judge == true{
                
            }
        }
    }
}



