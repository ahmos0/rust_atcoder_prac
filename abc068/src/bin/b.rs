use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut t = 1;
    while n > t{
        t *= 2;
    }
    if n < t{
        println!("{}",t/2);
    } else {
        println!("{}",t);
    }
}
