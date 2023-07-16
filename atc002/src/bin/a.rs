use proconio::input;
fn main() {
    input! {
        r: usize,
        c: usize,
        s: (usize, usize),
        g: (usize, usize)
    }
    println!("{:?} {:?}",s, g);
}
