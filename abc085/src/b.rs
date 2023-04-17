use proconio::input;

fn main(){
    input!{
        n: usize,
        mut arr: [usize; n],
    }
    arr.sort_by(|a, b| b.cmp(a));
    let mut count = 0;
    let mut tmp = 0;
    for i in 0..n{
        if i == 0{
            tmp = arr[i];
            count +=1;
        } else if arr[i] != tmp{
            tmp = arr[i];
            count += 1;
        }
    }
    println!("{}",count);
}