use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut max = 0;
    let mut count = 0;
    for i in 0..s_c.len() {
        if s_c[i] == 'A' || s_c[i] == 'C' || s_c[i] == 'G' || s_c[i] == 'T' {
            count += 1;
            if count > max {
                max = count;
            }
        } else {
            count = 0;
        }
    }
    println!("{}",max);
}
