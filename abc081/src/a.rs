use proconio::input;
fn main() {
    let mut count = 0;
    input! {
        s: String,
    }
    for c in s.chars() {
        if c == '1'{
            count += 1;
        }
    }
    println!("{}",count);
    
}