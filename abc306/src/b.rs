use proconio::input;

fn main(){
    input!{
        mut arr: [i128; 64],
    }
    let mut total:i128 = 0;
    let mut two = 1;
    for i in 0..64{
        total += arr[i] * two;
        two *= 2;
    }
    println!("{}", total);
}