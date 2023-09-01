use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: String,
        q: usize,
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut tpp: Vec<(usize, char)> = Vec::with_capacity(0);
    let mut t2_impact = vec![false; n];
    let mut t3_impact = vec![false; n];
    let mut last_t2 = q;
    let mut last_t3 = q;

    for i in (0..q).rev() {
        input! {
            t: usize,
            x: usize,
            c: char,
        }
        if t == 1 {
            s_c[x - 1] = c;
            tpp.push((x - 1, c));
        } else if t == 2 {
            last_t2 = i;
            t2_impact[x - 1] = true;
        } else if t == 3 {
            last_t3 = i;
            t3_impact[x - 1] = true;
        }
    }

    for i in 0..n {
        if t2_impact[i] && last_t2 < last_t3 {
            s_c[i] = s_c[i].to_ascii_lowercase();
        } else if t3_impact[i] {
            s_c[i] = s_c[i].to_ascii_uppercase();
        }
    }

    for (i, c) in tpp.iter() {
        s_c[*i] = *c;
    }

    for c in s_c {
        print!("{}", c);
    }
}
