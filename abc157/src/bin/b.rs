use proconio::input;

fn main() {
    input! {
        a: [[usize;3];3],
        n: usize
    }
    let mut v: Vec<usize> = Vec::new();
    let mut judge: Vec<bool> = vec![false; 9];
    for i in 0..3 {
        for j in 0..3 {
            v.push(a[i][j]);
        }
    }
    for i in 0..n {
        input! {
            b: usize
        }
        for j in 0..9 {
            if v[j] == b {
                judge[j] = true;
            }
        }
    }
    if judge[0] && judge[1] && judge[2] {
        println!("Yes");
    } else if judge[3] && judge[4] && judge[5] {
        println!("Yes");
    } else if judge[6] && judge[7] && judge[8] {
        println!("Yes");
    } else if judge[0] && judge[3] && judge[6] {
        println!("Yes");
    } else if judge[1] && judge[4] && judge[7] {
        println!("Yes");
    } else if judge[2] && judge[5] && judge[8] {
        println!("Yes");
    } else if judge[0] && judge[4] && judge[8] {
        println!("Yes");
    } else if judge[2] && judge[4] && judge[6] {
        println!("Yes");
    } else {
        println!("No");
    }
}
