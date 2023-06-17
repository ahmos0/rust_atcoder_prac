use std::num;

use proconio::input;
fn main(){
    input! {
        n : usize,
        d : i128,
        mut b: [[i128; 2]; n]
    }
    let mut array: Vec<Vec<i128>> = vec![vec![0; 2]; 1];
    array[0][0] = b[0][0];
    array[0][1] = b[0][1];
    let mut flag = false;
    println!("array {}",array.len());
    for i in 0..n{
        for j in 0..array.len(){
            if (((b[i][0] - b[j][0])*(b[i][0] - b[j][0])) + ((b[i][1] - b[j][1])*(b[i][1] - b[j][1]))) <= d*d {
                flag = true;
                println!("o j{} {}", b[j][0], b[j][1]);
                break;
            }else {
                flag = false;
                println!("x j{} {}", j, i);
            }
            
        }
        if flag == true{
            println!("Yes");
            array.push(vec![b[i][0], b[i][1]]);
        } else{
            println!("No");
        }
    }
}