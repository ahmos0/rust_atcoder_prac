use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    let mut rem = 0;
    rem = n % 5;
    if 0 <= rem && rem < 3 {
        println!("{}", (n/5) * 5);
    } else {
        println!("{}", (n/5 + 1) * 5);
    }
}
