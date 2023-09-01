use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v: Vec<(usize, usize)> = Vec::new();
    for i in 0..n{
        input! {
            a: usize
        }
        v.push((a, i+1));
    }
    v.sort();
    for i in 0..n{
        print!("{} ", v[i].1);
    }
    
}
