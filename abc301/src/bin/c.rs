use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut t_c: Vec<char> = t.chars().collect();
    let mut s_m: Vec<char> = Vec::new();
    let mut t_m: Vec<char> = Vec::new();
    let mut c1 = vec![0; 8];
    let mut c2 = vec![0; 8];
    s_c.sort();
    t_c.sort();
    for i in 0..s_c.len() {
        if s_c[i] == '@'
            || s_c[i] == 'a'
            || s_c[i] == 't'
            || s_c[i] == 'c'
            || s_c[i] == 'o'
            || s_c[i] == 'd'
            || s_c[i] == 'e'
            || s_c[i] == 'r'
        {
            match s_c[i] {
                'a' => c1[0] += 1,
                't' => c1[1] += 1,
                'c' => c1[2] += 1,
                'o' => c1[3] += 1,
                'd' => c1[4] += 1,
                'e' => c1[5] += 1,
                'r' => c1[6] += 1,
                '@' => c1[7] += 1,
                _ => panic!(),
            }
        } else {
            s_m.push(s_c[i]);
        }
        if t_c[i] == '@'
            || t_c[i] == 'a'
            || t_c[i] == 't'
            || t_c[i] == 'c'
            || t_c[i] == 'o'
            || t_c[i] == 'd'
            || t_c[i] == 'e'
            || t_c[i] == 'r'
        {
            match t_c[i] {
                'a' => c2[0] += 1,
                't' => c2[1] += 1,
                'c' => c2[2] += 1,
                'o' => c2[3] += 1,
                'd' => c2[4] += 1,
                'e' => c2[5] += 1,
                'r' => c2[6] += 1,
                '@' => c2[7] += 1,
                _ => panic!(),
            }
        } else {
            t_m.push(t_c[i]);
        }
    }
    s_m.sort();
    t_m.sort();
    if s_m.len() != t_m.len(){
        println!("No");
        return;
    } 
    for i in 0..s_m.len(){
        if s_m[i] != t_m[i]{
            println!("No");
            return;
        }
    }
    for i in 0..7 {
        if c1[i] != c2[i] {
            if c1[i] > c2[i] {
                c2[7] -= c1[i] - c2[i];
            } else {
                c1[7] -= c2[i] - c1[i];
            }
        }
        if c1[7] < 0 || c2[7] < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
