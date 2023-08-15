use proconio::input;

fn main(){
    let mut a = vec!["a".to_string(); 100];
    let mut tmp = 1000000000;
    let mut li = 0;
    input! {
        n :usize,
    }
    for i in 0..n{
        input!{
            b : String,
            num : i32
        }
        a[i] = b;
        if num < tmp {
            tmp = num;
            li = i;
        }
    }
    for i in li..n{
        println!("{}",a[i])
    }
    if li != 0{
        for i in 0..li{
            println!("{}",a[i]);
        }
    }

}