use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut ans = 0;
    for i in 0..s_c.len(){
        if s_c[i] == 'v'{
            ans += 1;
        } else if s_c[i] == 'w'{
            ans += 2;
        }
    }
    println!("{}", ans);
}
