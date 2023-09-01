use proconio::input;

fn main() {
    input! {
        n : u64
    }
    let mut b = 0;
    let mut ans: u64 = 10;
    let mut flag = false;
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            b = n / i;
            let digits: u64 = digit_count(b) as u64;
            if ans > digits {
                ans = digits;
                flag = true;
            } 
        }
    }
    if flag == false{
        let digits: u64 = digit_count(n) as u64;
        ans = digits;
    }
    println!("{}", ans);
}

fn digit_count(n: u64) -> usize {
    n.to_string().len()
}
