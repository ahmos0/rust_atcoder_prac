use proconio::input;
fn main() {
    input! {
        m:usize,
        d:[usize; m]
    }
    let mut sum = 0;
    let mut mid = 0;
    let mut tmp = 0;
    for i in 0..m{
        sum += d[i];
    }
    mid = (sum+1)/2;
    sum = 0;
    for i in 0..m{
        sum += d[i];
        if sum >= mid {
            println!("{}",i+1);
            tmp = sum - d[i];
            println!("{}", mid - tmp);
            break;
        }
    }
}
