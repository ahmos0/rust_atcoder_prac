use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        h: usize,
        w: usize,
        ar: [String; h]
    }
    let mut n = min(w,h);
    let mut visite:Vec<Vec<bool>> = vec![vec![false;w];h];
    let mut ans: Vec<usize> = vec![0;n];
    let mut ar_c: Vec<Vec<char>> = Vec::new();
    for i in 0..h{
        ar_c[i] = ar[i].chars().collect();
    }
    for i in 1..n{
        
    }
}
