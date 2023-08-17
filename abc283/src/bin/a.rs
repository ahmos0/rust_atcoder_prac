use proconio::input;
fn main() {
    input! {
        a:usize,
        b:usize
    }
    let mut ans = 1;
    for i in 0..b{
        ans = ans * a;
    }
    println!("{}", ans);
}
