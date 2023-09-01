use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: i32,
        k: i32,
        s: String,
        mut xy : [(i128,i128);m]
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut now = (0, 0);
    let mut st: HashSet<(i128, i128)> = HashSet::new();

    for &(nx, ny) in &xy {
        st.insert((nx, ny));
    }
    for i in 0..s_c.len() {
        if s_c[i] == 'R' {
            now.0 += 1;
            h -= 1;
        } else if s_c[i] == 'L' {
            now.0 -= 1;
            h -= 1;
        } else if s_c[i] == 'U' {
            now.1 += 1;
            h -= 1;
        } else if s_c[i] == 'D' {
            now.1 -= 1;
            h -= 1;
        }
        if h < 0 {
            println!("No");
            return;
        }

        if st.contains(&now) && k > h {
            st.remove(&now);
            h = k;
        }
    }
    println!("Yes");
}
