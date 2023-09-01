use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n]
    }

    let mut ans = 0;

    for i in 0..n {
        for j in i + 1..n {
            let mut flag = false;
            for k in 0..m {
                let mut found = false;
                for l in 0..n - 1 {
                    if (a[k][l] == i + 1 && a[k][l + 1] == j + 1) || (a[k][l] == j + 1 && a[k][l + 1] == i + 1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    flag = true;
                    break;
                }
            }
            if !flag {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
