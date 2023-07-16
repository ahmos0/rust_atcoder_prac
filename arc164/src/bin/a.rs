fn count_ways(n: u64, k: u64) -> bool {
    if k == 0 {
        return true;
    }

    if n == 0 {
        return false;
    }

    for i in (0..=n).rev() {
        if 3u64.pow(i as u32) <= k {
            if count_ways(i - 1, k - 3u64.pow(i as u32)) {
                return true;
            }
        }
    }

    false
}

fn main() {
    proconio::input! {
        t: usize,
        test_cases: [(u64, u64); t],
    }

    for (n, k) in test_cases {
        if count_ways(n, k) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
