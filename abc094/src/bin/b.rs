use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize
    }
    let mut l = 0;
    let mut r = 0;
    for i in 0..m{
        input! {
            a: usize
        }
        if a < x {
            l += 1;
        } else if a > x{
            r += 1;
        }
    }
    if l > r {
        println!("{}", r);
    } else {
        println!("{}", l);
    }
}
