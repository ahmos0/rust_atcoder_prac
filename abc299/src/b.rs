use proconio::input;

fn main(){
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }
    let mut rw: [usize; 100000] = [0; 100000];
    let mut count = 0;
    let mut num: [usize; 100000] = [0; 100000];
    for i in 0..n{
        if c[i] == t{
            println!("ci {}", c[i]);
            num[count] = r[i]; // r[i]noatai
            println!("ri {}", num[count]);
            rw[count] = i;  //
            println!("rw {}", rw[count]);
            count += 1;
            println!("count {}",count);
        } 
    }
    let mut max = 0;
    let mut out_num = 0;
    if count > 1 {
        for i in 0..=count{
            if max <  num[i]{
                max = num[i];
                out_num = rw[i] + 1;
            }
        }
        println!("{}", out_num);
    } else {
        println!("{}", rw[0]);
    }
}