use proconio::input;

fn main() {
    input! {
        n: i128,
        k: i128
    }
    let d = n%k;
    let m = k - (n%k);
    
    if m > d {
        println!("{}",d);
    } else {
        println!("{}",m);
    }
}
