use proconio::input;

fn main(){
    input!{
        N:i64,
        A:i64,
        B:i64,
    }
    let mut sum = 0;
    for i in 1..=N{
        let mut tmp = i;
        let mut tmp_sum = 0;
        while tmp > 0{
            tmp_sum += tmp % 10;
            tmp /= 10;
        }
        if A <= tmp_sum && tmp_sum <= B{
            sum += i;
        }
    }
    println!("{}",sum);
}