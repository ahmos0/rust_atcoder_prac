use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m]
    }
    let mut sum = 0;
    let mut count = 0;
    for i in 0..n{
        for j in 0..m{
            input! {
                a: i32
            }
            sum += a * b[j];
        }
        sum += c;
        if sum > 0{
            count += 1;
        }
        sum = 0;
    }
    println!("{}", count);
}
