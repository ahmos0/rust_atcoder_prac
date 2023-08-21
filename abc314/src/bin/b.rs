use proconio::input;

fn main() {
    input! {
        n : usize
    }
    let mut qua: Vec<usize> = Vec::with_capacity(0);
    let mut val: Vec<Vec<bool>> = vec![vec![false; 37]; n];
    let mut ans: Vec<(usize, usize)> = Vec::with_capacity(0);
    let mut min = 37;
    let mut count = 0;
    for i in 0..n {
        input! {
            mut a: usize
        }
        qua.push(a);
        for _j in 0..a {
            input! {
                mut b:usize
            }
            val[i][b] = true;
        }
    }
    input! {
        x:usize
    }
    for i in 0..n {
        if val[i][x] {
            ans.push((qua[i], i + 1));
            if min > qua[i] {
                min = qua[i];
            }
        }
    }
    ans.sort();
    for i in 0..ans.len(){
        if ans[i].0 == min {
            count += 1;
        }
    }
    println!("{}", count);
    for i in 0..ans.len() {
        if ans[i].0 == min {
            print!("{} ", ans[i].1);
        }
    }
}
