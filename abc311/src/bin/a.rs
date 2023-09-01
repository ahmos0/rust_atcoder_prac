use proconio::input;

fn main() {
    input! {
        n : usize,
        s : String
    }
    let mut s_char: Vec<char> = s.chars().collect();
    let mut flag: (bool, bool, bool) = (false, false, false);
    for i in 0..n {
        if flag == (true, true, true) {
            println!("{}", i);
            break;
        } else if s_char[i] == 'A' {
            flag.0 = true;
        } else if s_char[i] == 'B' {
            flag.1 = true;
        } else if s_char[i] == 'C' {
            flag.2 = true;
        }
        if flag == (true, true, true) {
            println!("{}", i+1);
            break;
        }
    }
}
