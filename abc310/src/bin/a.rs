use proconio::input;
fn main() {
    input! {
        n: usize,
        p: i128,
        q: i128,
        mut d: [i128; n]
    }
    let mut min = p;
    for i in 0..n{
        if min > q + d[i]{
            min = q + d[i];
        }
    }
    println!("{}", min);
}
