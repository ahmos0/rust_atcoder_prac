use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [String; h],
        b: [String; h],
    }

    for s in 0..h {
        for t in 0..w {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    if a[(i + h - s) % h].chars().nth((j + w - t) % w).unwrap() != b[i].chars().nth(j).unwrap() {
                        ok = false;
                    }                    
                }
                if !ok {
                    break;
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
