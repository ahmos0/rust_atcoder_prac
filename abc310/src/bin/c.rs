use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut ans = 0;
    let mut set: HashSet<String> = HashSet::new();
    for i in 0..n {
        let string_s: String = s[i].chars().collect();
        if !set.contains(&string_s) {
            ans += 1;
        }
        set.insert(string_s);
        let reverse_s: String = s[i].chars().rev().collect();
        set.insert(reverse_s);
    }
    println!("{}", ans);
}
