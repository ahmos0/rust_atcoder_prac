use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut count = 0;
    let mut flag = true;
    let mut tmp = 0;
    s_c.reverse();
    while flag {
        flag = false;
        for i in 0..s_c.len()-1{
            if s_c[i] == 'W' && s_c[i+1] == 'B'{
                flag = true;
                s_c[i] = 'B';
                s_c[i + 1] = 'W';
                count += 1;
            }  
        }
    }

    println!("{}",count);
}
