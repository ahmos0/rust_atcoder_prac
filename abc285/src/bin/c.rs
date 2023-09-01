use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let l = s.len();
    let mut res = 0;
    let mut add = 26;
    for i in 1..l {
        res += add;
        add *= 26;
    }
    let mut num = 0;
    for c in s.chars() {
        num *= 26;
        num += (c as u64) - ('A' as u64);
    }
    println!("{}", res + num + 1);
}
