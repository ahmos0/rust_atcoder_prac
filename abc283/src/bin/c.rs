use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut ans = 0;
    let mut s_chars: Vec<char> = s.chars().collect();

    while let Some(back) = s_chars.pop() {
        if back == '0' && s_chars.last() == Some(&'0') {
            s_chars.pop();
        }
        ans += 1;
    }

    println!("{}", ans);
}
