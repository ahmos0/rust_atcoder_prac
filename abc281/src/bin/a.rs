use proconio::input;
fn main() {
    input! {
       mut  n: i64,
    };
    while n > -1 {
        println!("{}", n);
        n -= 1;
    }
}
