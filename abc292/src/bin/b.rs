use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut t: Vec<(usize, bool)> = vec![(0,false); n];
    for i in 0..q{
        input! {
            a: usize,
            x: usize
        }
        if a == 1{
            t[x-1].0 += 1;
            if t[x-1].0 == 2{
                t[x-1].1 = true;
            }
        } else if a == 2{
            t[x-1].1 = true;
        } else if a == 3{
            if t[x-1].1 == true{
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
