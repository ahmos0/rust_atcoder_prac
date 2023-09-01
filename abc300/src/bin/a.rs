use proconio::input;
fn main() {
    input! {
        n : usize,
        a : i128,
        b : i128,
        mut c: [i128; n],
    }
    let mut sum = a + b;
    for i in 0..n {
        if sum == c[i] {
            println!("{}", i + 1);
            break;
        }
    }
}
