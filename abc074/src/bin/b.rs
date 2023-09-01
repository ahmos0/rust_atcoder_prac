use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        x: [i32; n]
    }
    let mut sum = 0;
    for i in 0..n {
        if (x[i] - 0).abs() > (x[i] - k).abs() {
            sum += (x[i] - k).abs() * 2;
        } else {
            sum += x[i].abs() * 2;
        }
    }
    println!("{}", sum);
}
