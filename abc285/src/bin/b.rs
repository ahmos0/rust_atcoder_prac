use proconio::input;
fn main() {
    input! {
        n : usize,
        s : String
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    for i in 1..n {
        for j in 1..=n {
            if j + i > n {
                println!("{}", j-1);
                break;
            } else if s_chars[j - 1] == s_chars[j + i - 1] {
                println!("{}", j-1);
                break;
            }
        }
    }
}
