use proconio::input;

fn main() {
    input! {
        n: usize,
        a: String
    }
    let result: Vec<char> = a.chars().collect();
    let mut out = String::new();
    for i in 0..result.len() {
        out.push(result[i]);
        out.push(result[i]);
    }
    println!("{}",out);
}