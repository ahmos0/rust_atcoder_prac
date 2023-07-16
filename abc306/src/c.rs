use proconio::input;

fn main(){
    input!{
        n : usize,
    }
    let mut arr_count: [usize; 100000] = [0; 100000];
    let mut result : [usize; 100000] = [0; 100000];
    //let mut result_index : [usize; 100000] = [0; 100000];
    let mut j = 0;
    for i in 0..3*n{
        input!{
            a : usize
        }
        arr_count[a-1] += 1;
        if arr_count[a-1] == 2{
            result[j] = a; 
            j += 1;
        }
    }
    for j in 0..n{
        println!("{}",result[j]);
    }
}