use proconio::input;

fn main() {
    input! {
        mut s: i32
    }
    let mut c = 0;
    let mut v: Vec<i32> = Vec::new();
    loop {
        if c == 0{
            v.push(s);
            c+=1;
        } else {
            if s % 2 == 0{
                s = s/2;
                c+=1;
            } else {
                s = 3*s + 1;
                c+=1;
            }
            for i in 0..v.len(){
                if s == v[i]{
                    println!("{}",c);
                    return;
                }
            }
            v.push(s);
        }
    }
}
