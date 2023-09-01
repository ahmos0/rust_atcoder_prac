use proconio::input;

fn main() {
    input! {
        a: String,
        b: String
    }
    let ab = format!("{}{}", a, b);
    if let Ok(al) = ab.parse::<i32>() {
        if al == ((al as f32).sqrt() as i32).pow(2){
            println!("Yes");
            return;
        }
    } 
    println!("No");
}
