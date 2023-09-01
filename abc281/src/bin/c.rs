use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n]
    }

    let mut sum = 0;
    let mut c = 0;
    let mut m = 0;
    let mut total = 0;
    let mut tmp = 0;
    let mut ans = 0;
    for i in 0..n {
        total += a[i];
    }
    m = t % total;
    for i in 0..n {
        sum += a[i];
        if sum > m {
            ans = a[i] - (sum - m);
            c = i + 1;
            break;
        }
    }
    println!("{} {}", c, ans);
}
