use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        s:[String; n]
    }
    let mut s_char: Vec<Vec<char>> = vec![Vec::new(); n];
    let mut count = 0;
    let mut max = 0;
    for i in 0..n {
        s_char[i] = s[i].chars().collect();
    }
    for i in 0..d {
        for j in 0..n {
            if s_char[j][i] == 'x'{
                count = 0;
                break;
            } else if j == n-1{
                count += 1;
            } 
        }
        if max < count {
            max = count;
        }
    }
    println!("{}", max);
}
