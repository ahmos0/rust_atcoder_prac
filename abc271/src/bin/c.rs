use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v: Vec<bool> = vec![false; 1000000010];
    let mut a: Vec<usize> = Vec::new();
    let mut count = 0;
    let mut tail1 = 0;
    let mut t_n1 = 0;
    let mut t_n2 = 0;
    let mut tail2 = 0;
    for i in 0..n{
        input! {
            x: usize
        }
        a.push(x);
        v[x-1] = true;
    }
    tail1 = a[a.len()-1];
    t_n1 = a.len()-1;
    t_n2 = a.len()-2;
    tail2 = a[a.len()-2];
    for i in 0..n{
        if a[i] == i + 1{
            v[i] = false;
            count += 1;
        } else if v[tail1] && v[tail2]{
            v[tail1] = false;
            v[tail2] = false;
            t_n1 -= 2;
            t_n2 -= 2;
            count+=1;
        } else {
            break;
        }
    }
    println!("{}", count);
}
