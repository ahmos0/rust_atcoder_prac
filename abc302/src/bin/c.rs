use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [String; n],
    }

    s.sort();

    let mut found = false;
    for perm in s.iter().permutations(n) {
        let mut ok = true;
        for i in 0..n - 1 {
            let cnt = perm[i]
                .chars()
                .zip(perm[i + 1].chars())
                .filter(|&(c1, c2)| c1 != c2)
                .count();
            if cnt != 1 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            found = true;
            break;
        }
    }

    if !found {
        println!("No");
    }
}
