use proconio::input;

fn main(){
    input!{
        mut n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    let mut count_a = 0;
    let mut count_b = 0;
    for i in 0..n{
        for j in 0..n{
            if a[i][j] == 1{
                count_a += 1;
            }
            if b[i][j] == 1{
                count_b += 1;
            }
        }
    }
    if count_a > count_b {
        println!("No");
        return;
    } else if count_b == n*n || count_a == 0{
        println!("Yes");
        return;
    }
    let mut flag = false;
    let mut t: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[n - 1 - j][i] = a[i][j];
        }
        for i in 0..n {
            for j in 0..n {
                if b[i][j] == 0 && t[i][j] == 1{
                    flag = false;
                    break; 
                } else if b[i][j] == 1 && t[i][j] == 1{
                    flag = true;
                }
            }
            if flag == false{
                break;
            }
        }
    }
    if flag == false{
        println!("No");
    } else if flag == true{
        println!("Yes");
    }
    


}