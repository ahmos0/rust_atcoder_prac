use proconio::input;

fn main(){
    input! {
        n:usize,
        m:usize,
        c: [String; n], //0 n-1
        d: [String; m], //0 m-1
        p: [i128; m+1]
    }
    let mut tmp = 0;
    let mut sum = 0;    
    
    for i in 0..n{
        for j in 0..m{
            tmp = 0;
            if c[i] == d[j]{
                //println!("{} {}", c[i], d[j]);
                sum += p[j+1];
                break;
                //println!("{}",tmp);
            } else if j == m-1{
                //println!("{} {}", c[i], d[j]);
                tmp = p[0];
                //println!("{}",tmp);
                sum += tmp;
                break;
            }
            //sum += tmp;
        }
        //sum += tmp;
    }
    println!("{}", sum);
}
