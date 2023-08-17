use proconio::input;
fn main() {
    input! {
        n: usize,
        mut array: [String;n]
    }
    array.reverse();
    for i in 0..array.len(){
        println!("{}", array[i]);
    }
}
