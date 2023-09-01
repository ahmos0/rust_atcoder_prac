use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        n:usize,
        x:[i128; n]
    }
    let max_value = x.iter().cloned().max().unwrap();
    let mut ans: i128 = 99999999999999;
    let mut tmp: i128 = 0;
    for i in 0..max_value {
        tmp = 0;
        for j in 0..n{
            tmp += (x[j] - (i+1)).pow(2);
        }
        ans = min(ans, tmp);
    }
    println!("{}", ans);
}
