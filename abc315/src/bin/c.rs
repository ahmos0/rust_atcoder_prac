use proconio::input;

fn main() {
    input! {
        n: usize,
        fs: [(i128, i128); n]
    }

    let mut max: Vec<Vec<i128>> = vec![vec![0; 2]; n];
    let mut tmp = 0;
    let mut a_max = 0;
    let mut b_max = vec![0; 2];
    for i in 0..n {
        let (a, b) = fs[i];
        if max[(a as usize) - 1][0] < b {
            tmp = max[(a as usize) - 1][0];
            max[(a as usize) - 1][0] = b;
            if max[(a as usize) - 1][1] < b {
                max[(a as usize) - 1][1] = tmp;
            }
        } else if max[(a as usize) - 1][1] < b {
            max[(a as usize) - 1][1] = b;
        }
    }
    
    for i in 0..n {
        if i == 0 {
            b_max[0] = max[i][0];
        } else {
            if b_max[0] < max[i][0] {
                tmp = b_max[0];
                b_max[0] = max[i][0];
                if b_max[1] < tmp {
                    b_max[1] = tmp;
                }
            } else if b_max[1] < max[i][0]{
                b_max[1] = max[i][0];
            }
        }
        if a_max < (max[i][0] + (max[i][1]/2)) {
            a_max = max[i][0] + (max[i][1]/2);
        }
    }
    tmp = b_max[0] + b_max[1];
    if tmp > a_max {
        println!("{}", tmp);
    } else {
        println!("{}", a_max);
    }
}
