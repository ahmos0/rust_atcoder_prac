use proconio::input;

fn main() {
    input! {
        mut a: i128,
        mut b: i128,
        mut c: i128
    }
    let mut flag = true;
    let mut ad = 0;
    let mut bd = 0;
    let mut cd = 0;
    let mut count = 0;
    if a % 2 == 1 || b % 2 == 1 || c % 2 == 1{
        println!("0");
            return;
    }
    while flag{

        if a == b && b == c{
            println!("-1");
            return;
        }
        if (a/2 %2) == 1 && (b/2) %2 == 1 && (c/2) %2 == 1{
            count += 1;
            ad = a/2;
            bd = b/2;
            cd = c/2;
            a = bd + cd;
            b = ad + cd;
            c = ad + bd;
        } else if (a / 2) % 2 == 1 ||(b / 2) % 2 == 1 || (c / 2) % 2 == 1{
            count+=1;
            println!("{}", count);
            return;
        } else {
            count += 1;
            ad = a/2;
            bd = b/2;
            cd = c/2;
            a = bd + cd;
            b = ad + cd;
            c = ad + bd;
        }
    }

}
