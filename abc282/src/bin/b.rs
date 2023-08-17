use proconio::input;
fn main() {
    input! {
        n:usize,
        m:usize,
        arr : [String;n]
    }
    let mut char_arr: Vec<Vec<char>> = vec![vec![' '; m]; n];
    for i in 0..n {
        let chars: Vec<char> = arr[i].chars().collect();
        char_arr[i].copy_from_slice(&chars);
    }
    let mut count = 0;
    for i in 0..n{
        for j in i+1..n{
            for k in 0..m{
                if char_arr[i][k] == 'x' && char_arr[j][k] == 'x'{
                    break;
                } else if k == m-1{
                    count += 1;
                }
            }
        }
    }
    println!("{}",count);
}
