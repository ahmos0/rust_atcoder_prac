use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut new_string = String::new();
    for c in s.chars() {
        if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' {
            new_string.push(c);
        }
    }
    println!("{}", new_string);
}
