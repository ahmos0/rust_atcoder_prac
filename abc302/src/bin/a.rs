use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128
    }
    let mut count = 0;
    if a % b != 0{
        count = a/b + 1;
    } else {
        count = a/b;
    }
    println!("{}", count);
}
