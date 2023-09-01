use proconio::input;

fn main() {
    input! {
        h : usize,
        w : usize,
    }
    let mut count = 0;
    for i in 0..h{
        input! {
            s : String
        }
        let mut s_c: Vec<char> = s.chars().collect();
        for j in 0..s_c.len(){
            if s_c[j] == '#'{
                count +=1;
            }
        }
    }
    println!("{}",count);
}
