use proconio::input;


fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }
    
    let mut ans = 0;
    
    for i in 0..n{
        ans += a[i]-1;
    }
    
    println!("{}", ans);
}
