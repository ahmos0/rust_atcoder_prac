use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    if s_c.len() != 8{
        println!("No");
        return;
    }
    for i in 0..s.len(){
        if i == 0 && !s_c[i].is_ascii_uppercase(){
            println!("No");
            return;
        } else if 1 <= i && i <= 6 {
            if i == 1 && s_c[i] == '0'{
                println!("No");
                return;
            }
            match s_c[i]{
                '0' |'1' | '2' | '3' |'4' |'5' | '6' | '7' |'8'|'9' => continue,
                _ => {println!("No"); return;},
            }
        } else if i == 7 && !s_c[i].is_ascii_uppercase(){
            println!("No");
            return;
        }
    }
    println!("Yes");
}
