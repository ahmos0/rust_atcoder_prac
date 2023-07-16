use proconio::input;
//use petgraph;
//use std::collections::HashMap;
fn main() {
    input! {
        a:usize,
        b:usize
    }
    let mut flag = false;
    if a == 1 && b == 2 {
        flag = true;
    } else if a == 2 && b == 3 {
        flag = true;
    } else if a == 4 && b == 5 {
        flag = true;
    } else if a == 5 && b == 6 {
        flag = true;
    } else if a == 7 && b == 8 {
        flag = true;
    } else if a == 8 && b == 9 {
        flag = true;
    } 
    if flag{
        println!("Yes");
    } else {
        println!("No");
    }
} 
