use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [i32; n],
    }
    
    h.sort();
    
    let mut ans = std::i32::MAX;
    
    for l in 0..=n-k {
        let r = l + k - 1;
        ans = min(ans, h[r] - h[l]);
    }
    
    println!("{}", ans);
}
