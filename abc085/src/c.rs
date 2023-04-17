use proconio::input;

fn main(){
    input!{
        n : i64,
        y: i64,
    }
    let mut a = -1;
    let mut b = -1;
    let mut c = -1;
    let mut total = 0;

    for i in 0..=n{
        for j in 0..=n{
            if i+j > n {
                break;
            }
            let mut tmp = n-i-j;
            total = 10000*i+5000*j+1000*tmp;
            if total == y{
                a = i;
                b = j;
                c = tmp;
            }
        }
    }
    println!("{} {} {}", a, b, c);
}