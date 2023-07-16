use proconio::input;
//use petgraph;
//use std::collections::HashMap;
fn main() {
    input!{
        n : usize,
        mut a: [[i128; n]; n]
    }
    let mut new_a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            new_a[j][(n - 1) - i] = a[i][j];
        }
    }
    println!("{:?}", new_a);
}
