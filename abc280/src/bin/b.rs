use proconio::input;

fn main() {
    input! {
        n : usize,
        s : [i128; n]
    }
    let mut ans: Vec<i128> = Vec::with_capacity(0);
    let mut sum = 0;
    for i in 0..n{
        if i == 0{
            ans.push(s[i]);
            sum += s[i];
        } else {
            ans.push(s[i]-sum);
            sum += s[i]-sum;
        }
    }
    for i in 0..ans.len(){
        print!("{} ",ans[i]);
    }
}
