use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        c: usize
    }
    let mut count = 1;
    let mut first = 0;
    let mut tmp = 0;
    loop { 
        tmp = (a * count) % b;
        if tmp == c {
            println!("YES");
            return;
        } else if count == 1{
            first = tmp;
            count += 1;
        } else if tmp == first{
            println!("NO");
            return;
        } else {
            count += 1;
        }
    }
}
