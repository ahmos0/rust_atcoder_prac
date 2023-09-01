use proconio::input;

fn main() {
    input! {
        n: usize,
        q: i128,
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut rotate = 0;
    for i in 0..q {
        input! {
            t: usize,
            x: i128
        }
        if t == 1 {
            rotate = (rotate + x % n as i128) % n as i128;
        } else if t == 2 {
            let index = ((x as i128 - 1 - rotate + n as i128) % n as i128) as usize;
            println!("{}", s_c[index]);
        }
    }
}
