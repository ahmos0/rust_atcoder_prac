use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize
    }
    let mut now = 1;
    let mut count = 0;
    while now < b {
        if count == 0 {
            now = a;
            count += 1;
        } else {
            now = (now - 1) + a;
            count += 1;
        }
    }
    println!("{}", count);
}
