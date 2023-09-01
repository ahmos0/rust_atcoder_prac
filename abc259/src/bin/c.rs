use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    if s == t {
        println!("Yes");
        return;
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    let mut one = 'a';
    let mut two = 'a';
    if s_c.len() > t_c.len() {
        println!("No");
        return;
    }
    for i in 0..t_c.len() {
        if s_c.len() <= i {
            s_c.push('8');
        }
        if i == 0 {
            if s_c[i] == t_c[i] {
                one = s_c[i];
            } else {
                println!("No");
                return;
            }
        } else if i == 1 {
            if s_c[i] == t_c[i] {
                two = s_c[i];
            } else {
                println!("No");
                return;
            }
        } else if 2 <= i {
            if s_c[i] == t_c[i] {
                one = two;
                two = s_c[i];
            } else {
                if one == two && two == t_c[i] {
                    s_c.insert(i, two);
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
