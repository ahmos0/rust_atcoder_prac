use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let fh = 400;
    let mut counts = [0; 8];
    let mut count = 0;
    let mut red = 0;
    for _ in 0..n {
        input! {
            a: usize
        }
        if a < fh * 8 {
            let range_index = a / fh;
            counts[range_index] += 1;
        } else {
            red += 1;
        }
    }

    for &c in counts.iter().take(8) {
        if c != 0 {
            count += 1;
        }
    }
    if count == 0 {
        println!("1 {}",red);
    }else {
        println!("{} {}",count, count + red);
    }
}

