use proconio::input;
fn main() {
    input! {
        n:usize,
        m: usize,
    }
    let mut c = 0;
    let mut arr1: [i128; 200] = [0; 200];
    let mut arr2: [i128; 200] = [0; 200];
    let mut tmp1 = 0;
    let mut tmp2 = 0;
    for i in 0..n{
        input!{
            p: i128,
            c: usize
        }
        for j in 0..c{
            if i == 0{
                input! {
                    a : i128
                }
                arr1[j] =  a;
            } else {
                input! {
                    a : i128
                }
                arr2[j] = a;
            }
        }
        if i == 0{
            tmp1 = c;
        }
        if i != 0{
            for j in 0..c{
                
            }
        }
    }
}
