use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [f64; n]
    }
    let mut ans = 0.0;
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ans = v[0];
    for i in 1..n {
        ans = (ans + v[i]) / 2.0;
    }
    println!("{}", ans);
}
