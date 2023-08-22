use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m]
    }
    a.sort();
    b.sort_by(|a, b| b.cmp(&a));

    let mut l = 0;
    let mut r = 1_000_000_001;

    while r - l > 1 {
        let mid = l + (r - l) / 2;
        let mut seller = 0;

        for &price in &a {
            if price <= mid {
                seller += 1;
            }
        }
        let mut buyer = 0;
        for &price in &b {
            if price >= mid {
                buyer += 1;
            }
        }

        if seller >= buyer {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", r);
}
