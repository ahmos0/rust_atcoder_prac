use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let m: usize = 1000000007;
    let mut div: Vec<usize> = vec![0; n + 1];
    let mut qa = 1;
    let mut tmp: usize = 0;
    for i in 2..=n {
        tmp = i;
        for j in 2..=(tmp as f64).sqrt() as usize {
            while tmp % j == 0 {
                div[j - 1] += 1;
                tmp /= j;
            }
        }
        if tmp != 1 {
            div[tmp - 1] += 1;
        }
    }
    for i in 0..div.len() {
        if div[i] != 0 {
            qa = (qa * (div[i] + 1)) % m; 
        }
    }
    println!("{}", qa);
}
