use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut v: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    let mut t = 0;
    for i in 0..n{
        input! {
            a: usize
        }
        v.push((i+1, a));
    }
    while count <= n{
        if v[t].1 == 2{
            count += 1;
            println!("{}", count);
            return;
        } else {
            t = v[t].1 - 1;
            count += 1;
        }
    }
    println!("-1");
}
