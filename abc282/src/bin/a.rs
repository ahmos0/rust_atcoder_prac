use proconio::input;
fn main() {
    input! {
        n:usize
    }
    for i in 0..n{
        print!("{}", ('A' as u8 + i as u8) as char);
    }
    println!();
}
