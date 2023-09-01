use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        K: i32,
        N: usize,
        mut A: [i32; N],
    }

    A.sort(); 

    let mut ans = A[N - 1] - A[0];
    for i in 1..N {
        let i_to_start = K - A[i];
        let start_to_i1 = A[i - 1];
        ans = min(ans, i_to_start + start_to_i1);
    }
    
    println!("{}", ans);
}
