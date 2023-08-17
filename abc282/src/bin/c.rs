use proconio::input;
fn main() {
    input! {
        n:usize,
        mut s: String
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut flag = false;
    for i in 0..n{
        if !flag && s_chars[i] == ','{
            s_chars[i] = '.';
        } else if !flag && s_chars[i] == '\"'{
            flag = true;
        }  else if flag && s_chars[i] == '\"'{
            flag = false;
        }
    }
    for i in 0..n{
        print!("{}",s_chars[i]);
    }
    println!();
}
