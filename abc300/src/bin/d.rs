/*use proconio::input;

fn main() {
    input! {
        n:usize
    }
    let mut tmp = 0;
    let mut l = 0;
    let mut ans = 0;
    let mut flag = false;
    let mut flag2 = false;
    for i in 300..=n {
        let mut count: Vec<usize> = Vec::with_capacity(0);
        let mut k = 0;
        let mut total = 0;
        tmp = i;
        for j in 2..=(tmp as f64).sqrt() as usize {
            while tmp % j == 0 {
                flag = true;
                if l == 0 {
                    count.push(1);
                    l += 1;
                    total += 1;
                    tmp /= j;
                } else {
                    count[k] += 1;
                    total += 1;
                    tmp /= j;
                }
            }
            if flag{
                k += 1;
                flag = false;
            }
            l = 0;
        }

        if total == 5 && count[0] == 2 && count[1] == 1 && count[2] == 2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}*/
use proconio::input;

fn main(){
    input! {
        n: usize
    }
    let mut tmp = 0;
    let mut count = 0;
    //let mut flag = false;
    for i in 2..=((n as f64).powf(1.0 / 5.0) as usize){
        if n % (i*i) == 0{
            println!("i is {}", i);
            tmp = n / (i*i);
            println!("tmp is {}", tmp);
            for j in i..=((tmp as f64).powf(1.0 / 3.0) as usize){
                
                if tmp % j == 0{
                    println!("i is {} j is {}", i, j);
                    tmp = tmp/j;
                    for k in j+1..=((tmp as f64).powf(1.0 / 2.0) as usize){
                        if tmp == k*k{
                            println!("i is {} j is {} k is {}", i,j,k);
                            count+=1;
                            break; 
                        }
                    }
                }
            }
        }
    }
    println!("{}",count);
}
