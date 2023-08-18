use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize
    }
    if a == b / 2 || (a * 2 + 1 == b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
