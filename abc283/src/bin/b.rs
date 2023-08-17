use proconio::input;
fn main() {
    input! {
        n : usize,
        mut a : [i128; n],
        q : usize
    }
    for i in 0..q {
        input! {
            t : usize
        }
        match t {
            1 => {
                input! {
                    k: usize,
                    x: i128,
                }
                a[k-1] = x;
            }
            2 => {
                input! {
                    k : usize
                }
                println!("{}",a[k-1]);
            }
            _ => unreachable!(),
        }
    }
}
