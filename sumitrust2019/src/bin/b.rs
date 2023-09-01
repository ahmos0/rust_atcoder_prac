use proconio::input;

fn main() {
    input! {
        n: i32
    }
    for i in 0..=n{
        if (i as f32 * 1.08) as i32 == n{
            println!("{}", i);
            return
        } 
    }
    println!(":(");
    
}
