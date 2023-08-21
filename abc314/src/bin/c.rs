use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        s:String
    }
    let mut sc: Vec<char> = s.chars().collect();
    let mut tv: Vec<Vec<char>> = vec![Vec::new(); m]; 
    let mut num: Vec<usize> = Vec::with_capacity(0);
    let mut tmp: usize = 0;
    for i in 0..n {
        input! {
            a:usize
        }
        num.push(a);
        tv[a-1].push(sc[i]);
    }
    for i in 0..m {
        tv[i].rotate_right(1);
        tv[i].reverse();
    }
    let mut c = 'a';
    for i in 0..n{
        tmp = num[i];
        if let Some(ch) = tv[tmp-1].pop() {
            c = ch;
        }
        print!("{}", c);
    }
}
