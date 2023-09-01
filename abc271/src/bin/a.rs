use proconio::input;

fn main() {
    input! {
        mut n: usize
    }
    let hex_string = format!("{:02X}", n);
    println!("{}", hex_string);
}
