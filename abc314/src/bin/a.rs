use proconio::input;
fn main() {
    input! {
        n : usize
    }
    let pi = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    let mut s_pi: Vec<char> = pi.chars().collect();
    for i in 0..=n+1 {
        print!("{}", s_pi[i]);
    }
    println!();
}
