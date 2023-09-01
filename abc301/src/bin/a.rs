use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut tc = 0;
    let mut ac = 0;
    let mut v = 'n';
    for i in 0..n{
        if s_c[i] == 'T'{
            tc += 1;
            if n/2 == tc && v == 'n'{
                v = 'T';
            }
        } else {
            ac += 1;
            if n/2 == ac && v == 'n'{
                v = 'A';
            }
        }
    } 
    if tc == ac {
        println!("{}", v);
    } else if tc > ac{
        println!("T");
    } else {
        println!("A");
    }
}
