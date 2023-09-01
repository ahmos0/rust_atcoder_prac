use std::cmp::min;
use std::iter::repeat;
use proconio::input;

fn main() {
    let inf: i128 = 1e18 as i128;

    input! {
        n: usize,
        districts: [(i128, i128, i128); n],
    }

    let z_sum: i128 = districts.iter().map(|(_, _, z)| z).sum();

    let mut dp: Vec<i128> = repeat(inf).take((z_sum + 1) as usize).collect();
    dp[0] = 0;

    for &(x, y, z) in &districts {
        let w = i128::max(0, (x + y) / 2 + 1 - x);
        for j in (z..=z_sum).rev() {
            dp[j as usize] = min(dp[j as usize], dp[(j - z) as usize] + w);
        }
    }

    let mut ans = inf;
    for j in (z_sum / 2 + 1)..=z_sum {
        ans = min(ans, dp[j as usize]);
    }

    println!("{}", ans);
}
