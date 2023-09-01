use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut total = 0;
    let mut b_c = 0;
    
    for i in 0..s_c.len(){
        if s_c[i] == 'a' && a + b >= total + 1{
            println!("Yes");
            total += 1;
        } else if s_c[i] == 'b' && a + b >= total + 1 && b >= b_c + 1{ 
            println!("Yes");
            total += 1;
            b_c += 1;
        } else {
            println!("No");
        }
    }
}
