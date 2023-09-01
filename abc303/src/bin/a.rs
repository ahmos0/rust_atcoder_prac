use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        t: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    let mut flag = false;
    for i in 0..n{
        if s_c[i] == t_c[i]{
            flag = true;
        } else if ((s_c[i] == 'l' || t_c[i] == 'l') &&  (s_c[i] == '1'|| t_c[i] == '1')) || ((s_c[i] == '0' || t_c[i] == '0') &&  (s_c[i] == 'o'|| t_c[i] == 'o')){
            flag = true;
        } else {
            flag = false;
            break;
        }
    }
    if flag{
        println!("Yes");
    } else {
        println!("No");
    }
}
