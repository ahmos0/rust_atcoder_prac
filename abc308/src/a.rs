use proconio::input;

fn main(){
    let mut tmp = 0;
    for i in 0..8{
        input! {
            s: i128
        }
        if i == 0{
            tmp = s;
        } else {
            if tmp <= s{
                tmp = s;
            } else {
                println!("No");
                return;
            }
        }
        if 100 > s || s > 675{
            println!("No");
            return;
        }
        if s%25 != 0{
            println!("No");
            return;
        }
    }
    println!("Yes");
}