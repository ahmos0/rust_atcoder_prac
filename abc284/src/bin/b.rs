use proconio::input;
fn main() {
    input! {
        t : usize
    }
    for i in 0..t {
        let mut count = 0;
        input! {
            n:usize,
            a:[i128; n]
        }
        for j in 0..a.len(){
            if a[j] % 2 == 1{
                count += 1;
            }
        }
        println!("{}",count);
    }
}
