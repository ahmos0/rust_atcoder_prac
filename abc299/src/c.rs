use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut tmp = 0;
    let mut max_val = 0;
    for i in 0..s_c.len() {
        if s_c[i] == '-' {
            max_val = max(max_val, i - tmp);
            tmp = i;
        } 
    }
    tmp = 0;
    s_c.reverse();
    for i in 0..s_c.len() {
        if s_c[i] == '-' {
            max_val = max(max_val, i - tmp);
            tmp = i;
        } 
    }
    if max_val <= 0{
        println!("-1");
    } else {
        println!("{}", max_val-1);
    }
}
