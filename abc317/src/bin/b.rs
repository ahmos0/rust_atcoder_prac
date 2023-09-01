use proconio::input;

fn main() {
    input! {
        n: i128,
        mut a: [i128; n]
    }
    a.sort();
    let mut tmp = 0;
    for i in 0..n {
        if i == 0 {
            tmp = a[i as usize];
        } else {
            if a[i as usize] - tmp == 2 {
                println!("{}", tmp + 1);
                return;
            } else {
                tmp = a[i as usize];
            }
        }
    }
}
