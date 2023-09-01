use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut v:Vec<Vec<char>> = Vec::new();
    for i in 0..h{
        input! {
            s: String
        }
        v[i] = s.chars().collect();
    }
    for i in 0..h{
        
    }

}
