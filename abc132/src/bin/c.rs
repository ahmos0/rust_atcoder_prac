use proconio::input;

fn main() {
    input! {
        n: usize,
        mut  d: [usize; n]
    }
    d.sort();
    let mid = n/2;
    let mut count = 0;
    for i in d[mid-1]..=d[mid]{
        count += 1;
    }
    println!("{}", count-1);
}
