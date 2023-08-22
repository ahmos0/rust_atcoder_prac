use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut max = 0;
    let mut first = 0;
    let mut count = 0;
    for i in 0..n {
        input! {
            a: i32
        }
        if i == 0 {
            first = a;
        }
        if max == a {
            count += 1;
        }
        if a > max {
            max = a;
        }
    }
    if count == 0 && max == first {
        println!("0");
    } else {
        println!("{}", max - first + 1);
    }
}
