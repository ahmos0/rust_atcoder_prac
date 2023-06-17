use proconio::input;

fn main(){
    input!{
       mut  n : i128
    }
    if n < 1000{
        println!("{}",n);
    } else if n >= 1000 && n < 10000{
        let a = n % 10;
        n = n - a;
        println!("{}",n);
    } else if n >= 10000 && n < 100000{
        let a = n % 100;
        n = n - a;
        println!("{}",n);
    }else if n >= 100000 && n < 1000000{
        let a = n % 1000;
        n = n - a;
        println!("{}",n);
    } else if n >= 1000000 && n < 10000000{
        let a = n % 10000;
        n = n - a;
        println!("{}",n);
    } else if n >= 10000000 && n < 100000000{
        let a = n % 100000;
        n = n - a;
        println!("{}",n);
    } else if n >= 100000000 && n < 1000000000{
        let a = n % 1000000;
        n = n - a;
        println!("{}",n);
    } 
}