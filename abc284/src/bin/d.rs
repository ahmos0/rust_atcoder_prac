use proconio::input;

fn main() {
    input! {
        t: usize,
        test: [usize; t]
    }
    let mut tmp = 0;
    for i in 0..t {
        for j in 2..=((test[i] as f64).powf(1.0 / 3.0) as usize) {
            if test[i] % j == 0 {
                tmp = test[i] / j;
                tmp = (tmp as f64).sqrt() as usize;
                if tmp * tmp * j == test[i] {
                    println!("{} {}", tmp, j);
                    break;
                } else {
                    println!("{} {}", j, test[i] / (j * j));
                    break;
                }
            }
        }
    }
}
