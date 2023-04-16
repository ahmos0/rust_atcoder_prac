use proconio::input;

fn main(){
    input!{
        N : usize,
        mut arr : [usize; N],
    }
    arr.sort_by(|a, b| b.cmp(a));
    let mut alice = 0;
    let mut bob = 0;
    let mut margin = 0;
    for i in 0..N{
        if i == 0 || i % 2 == 0{
            alice += arr[i];
        } else {
            bob += arr[i];
        }
    }
    margin = alice - bob;
    println!("{}",margin);
    
}