use proconio::input;

fn main(){
    input!{
        n : usize,
        mut arr: [usize; n],
    }
    let mut v = vec![0; n * 2 + 1];
    for i in 0..n{
        v[i * 2 + 1] = v[arr[i]] + 1;
        v[i * 2 + 2] = v[arr[i]] + 1;
    }
}