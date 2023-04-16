use proconio::input;

fn main(){
    input!{
        A:i64,
        B:i64,
        C:i64,
        X:i64,
    }
    let mut count = 0;
    for i in 0..=A{
        for j in 0..=B{
            for k in 0..=C{
                if 500*i+100*j+50*k == X{
                    count +=1;
                }
            }
        }
    }
    println!("{}", count);
}