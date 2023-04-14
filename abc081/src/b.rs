use proconio::input;

fn main(){
    
    input!{
        N :usize,
        mut a: [usize; N],
    }
    let mut count = 0;
    for i in 0..N{
        if a[i]%2 != 0{
            println!("{}",count);
            return 
        }
    }
    loop{
        count += 1;
        for i in 0..N{
            a[i]=a[i]/2;
            if a[i]%2 != 0{
                println!("{}",count);
                return 
            }
        }
    }

}