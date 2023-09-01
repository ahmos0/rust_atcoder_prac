use proconio::input;

fn main() {
    input! {
        n: i128
    }
    let mut sum = 0;
    for i in 1..=n{
        sum += i % n;
    }
    println!("{}", sum);
}
