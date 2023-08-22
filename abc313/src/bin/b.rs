use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        v:[(usize,usize);m]
    }
    let mut ok: Vec<bool> = vec![false; n];
    let mut tmp = 0;
    let mut count = 0;
    let mut res = 0;
    for i in 0..m{
        tmp = v[i].1;
        ok[tmp-1] = true;
    }
    for i in 0..n{
        if !ok[i] {
            count += 1;
            res = i + 1;
        }
    }
    if count == 0 || count > 1{
        println!("-1");
    } else {
        println!("{}", res);
    }
}
