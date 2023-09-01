use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    let mut tmp = 0;
    let mut count = 0;
    for i in 0..s_c.len(){
        tmp = i;
        if s_c[i] == t_c[0]{
            for j in 0..t_c.len(){
                if t_c[j] == s_c[tmp]{
                    count += 1;
                    tmp += 1;
                } else if t_c[j] != s_c[tmp]{
                    count = 0;
                    break;
                }
                if count == t_c.len(){
                    println!("Yes");
                    return;
                }
                if tmp >= s_c.len(){
                    count = 0;
                    break;
                }
            }
        }
    }
    println!("No");
}
