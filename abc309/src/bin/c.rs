use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i128,
        mut array: [(i128, i128); n]
    }
    let mut sum = 0;
    array.sort();
    
    for i in 0..n {
        sum += array[i].1;
    }
    if sum <= k {
        println!("1");
        return;
    } else {
        for i in 0..array.len() {
            if sum <= k {
                println!("{}", array[i - 1].0 + 1);
                return;
            }
            sum -= array[i as usize].1;
        }
    }
    println!("{}", array.last().unwrap().0 + 1);
}

