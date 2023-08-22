use std::vec;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i128;n]
    }
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..n{
        sum += a[i];
    }   
    a.sort();
    let mut b: Vec<i128> = vec![sum / (n as i128); n];
    for i in 0..(sum % n as i128){
        b[n - 1 - i as usize] += 1;
    }
    for i in 0..n{
        ans += (a[i]-b[i]).abs();
    }
    println!("{}", ans/2);
}
