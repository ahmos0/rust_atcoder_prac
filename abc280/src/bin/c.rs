use proconio::input;

fn main() {
    input! {
        s : String,
        t : String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    for i in 0..t.len(){
        if i == t_c.len()-1{
            println!("{}", t.len());
            break;
        }
        if s_c[i] != t_c[i] {
            println!("{}",i+1);
            break;
        }
    }
}
